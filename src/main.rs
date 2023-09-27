mod config;
mod job;
mod email;

use clap::Parser;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cfg = config::Config::parse();
    let email_sender = email::EmailSender::new(&cfg);
    let mut job_manager = job::JobManager::new(&cfg.query, cfg.paging);

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
        println!("shutting down");
    })?;
    println!("{}", cfg.query);

    let mut first_run: bool = cfg.first_run;
    while running.load(std::sync::atomic::Ordering::SeqCst) {
        if let Ok(new_jobs) = job_manager.fetch_new_jobs() {
            if first_run == false {
                first_run = true;
                continue;
            };

            for job in new_jobs {
                job_manager.display(&job);
                if let Err(err) = email_sender.send_email(&job, cfg.recipient.clone()) {
                    println!("sending email: {}", err);
                }
            }
        }
        std::thread::sleep(cfg.interval);
    }
    Ok(())
}
