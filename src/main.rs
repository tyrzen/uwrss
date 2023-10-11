extern crate tokio;

mod config;
mod job;
mod email;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let cfg = config::Config::parse();

    let email_sender = email::EmailSender::new(cfg.smtp_server.clone(), cfg.smtp_port, cfg.smtp_username.clone(), cfg.smtp_password.clone())?;
    let mut job_manager = job::JobManager::new(&cfg.query, cfg.paging)?;

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();


    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
        println!("shutting down");
    })?;
    println!("QUERY: {:?}", cfg.query);
    println!("INCLUDE_COUNTRIES: {:?}", cfg.include_countries);
    println!("{}", "-".repeat(100));

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        if let Ok(new_jobs) = job_manager.fetch_new_jobs().await {
            for job in new_jobs {
                if cfg.include_countries.contains(&job.country) {
                    if let Err(err) = email_sender.send_email(&job, cfg.smtp_username.clone(), cfg.recipient.clone()) {
                        println!("sending email: {}", err);
                    }
                }
            }
        }

        tokio::time::sleep(cfg.interval).await;
    }
    return Ok(());
}
