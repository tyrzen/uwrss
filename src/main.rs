mod config;
mod job;
mod email;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread::sleep;
use clap::Parser;
use ctrlc;

use config::Config;
use job::JobManager;
use email::EmailSender;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cfg = Config::parse();
    let email_sender = EmailSender::new(&cfg);
    let mut job_manager = JobManager::new(&cfg.query, cfg.paging);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("shutting down");
    })?;
    println!("{}", cfg.query);

    let mut first_run: bool = cfg.first_run;
    while running.load(Ordering::SeqCst) {
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
        sleep(cfg.interval);
    }
    Ok(())
}
