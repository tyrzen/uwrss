use std::error;
use std::sync::{self, atomic};

use clap::Parser;
use slog::{Drain, error, info, o, warn};
use slog_term;
use tokio;

#[macro_use]
mod config;
mod job;
mod email;


#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv::dotenv().ok();
    let cfg = config::Config::parse();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    let email_sender = email::EmailSender::new(cfg.smtp_server.clone(), cfg.smtp_port, cfg.smtp_username.clone(), cfg.smtp_password.clone())?;
    let mut job_manager = job::JobManager::new(&cfg.query, cfg.paging)?;

    let running = sync::Arc::new(atomic::AtomicBool::new(true));
    let r = running.clone();

    let ctrlc_log = log.clone();
    ctrlc::set_handler(move || {
        r.store(false, atomic::Ordering::SeqCst);
        warn!(ctrlc_log, "CTRL-C received");
    })?;

    info!(log, "Applied"; "query" => cfg.query);
    info!(log, "{}", "-".repeat(100));

    while running.load(atomic::Ordering::SeqCst) {
        match job_manager.fetch_new_jobs().await {
            Err(err) => { error!(log, "Fetching new jobs"; "error" => format!("{}", err)) }

            Ok(new_jobs) => {
                for job in new_jobs {
                    match email_sender.send_email(&job, &cfg.smtp_username, &cfg.recipient) {
                        Err(err) => { error!(log, "Sending email"; "error" => format!("{}", err)) }
                        Ok(_) => { info!(log, "Sending email"; "job" =>  &job.title) }
                    }
                }
            }
        };

        tokio::time::sleep(cfg.interval).await;
    }
    return Ok(());
}
