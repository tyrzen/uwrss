extern crate clap;
extern crate ctrlc;
extern crate html2text;
extern crate lettre;
extern crate reqwest;
extern crate rss;
extern crate scraper;
extern crate url;
extern crate dotenv;

use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::io::Cursor;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread::sleep;

use clap::Parser;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use lettre::message::header::To;
use lettre::message::{Mailboxes, SinglePart};
use lettre::transport::smtp::response::Response;
use rss::{Channel, Item};
use scraper::Html;
use url::Url;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", about = "Displays upwork jobs based on query")]
struct Config {
    #[clap(long, env = "INTERVAL", value_parser = | arg: & str | -> Result < std::time::Duration, std::num::ParseIntError > {Ok(std::time::Duration::from_secs(arg.parse() ?))})]
    interval: std::time::Duration,

    #[clap(long, env = "PAGING", default_value_t = 10)]
    paging: usize,

    #[clap(long, env = "QUERY", required = true)]
    query: String,

    #[clap(long, env = "SMTP_SERVER", required = true)]
    smtp_server: String,

    #[clap(long, env = "SMTP_PORT", required = true)]
    smtp_port: u16,

    #[clap(long, env = "SMTP_USERNAME", required = true)]
    smtp_username: String,

    #[clap(long, env = "SMTP_PASSWORD", required = true)]
    smtp_password: String,

    #[clap(long, env = "RECIPIENT", required = true)]
    recipient: String,
}

#[derive(Debug)]
struct JobListing {
    title: String,
    description: String,
}

fn parse_job(item: &Item) -> Result<JobListing, Box<dyn Error>> {
    let desc = Html::parse_fragment(item.description().unwrap_or_default());
    return Ok(JobListing {
        title: item.title().unwrap_or_default().to_string(),
        description: desc.html(),
    });
}

fn body_to_text(body: &str, width: usize) -> String {
    return html2text::from_read(Cursor::new(body), width).to_string();
}

fn build_url(query: &str, paging: usize) -> Result<String, url::ParseError> {
    let mut base_url = Url::parse("https://www.upwork.com/ab/feed/jobs/rss")?;
    base_url.query_pairs_mut()
        .append_pair("q", query)
        .append_pair("sort", "recency")
        .append_pair("paging", &format!("0;{}", paging));

    return Ok(base_url.to_string());
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let args = Config::parse();

    let email_sender = EmailSender {
        smtp_server: args.smtp_server,
        smtp_port: args.smtp_port,
        smtp_username: args.smtp_username,
        smtp_password: args.smtp_password,
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("shutting down");
    })?;

    let url = build_url(&args.query, args.paging)?;

    let mut seen_links = HashSet::new();
    let mut link_order = VecDeque::new();
    const DEFAULT_WIDTH: usize = 100;
    const MAX_CAPACITY: usize = 100;

    while running.load(Ordering::SeqCst) {
        let resp = reqwest::blocking::get(&url)?;
        let text = resp.text()?;
        let channel = Channel::read_from(text.as_bytes())?;

        for item in channel.items() {
            let link = item.link().unwrap_or_default().to_string();

            if seen_links.insert(link.clone()) {
                link_order.push_back(link);

                if seen_links.len() > MAX_CAPACITY {
                    if let Some(oldest_link) = link_order.pop_front() {
                        seen_links.remove(&oldest_link);
                    }
                }

                let job = parse_job(item)?;
                println!("Title: {}", job.title);
                println!("Description: {}", body_to_text(&job.description, DEFAULT_WIDTH));
                println!("{}", "-".repeat(DEFAULT_WIDTH));

                if let Err(e) = email_sender.send_email(&job, &args.recipient) {
                    println!("sending email: {}", e);
                }
            }
        }

        sleep(args.interval);
    }

    return Ok(());
}

struct EmailSender {
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
}

impl EmailSender {
    fn send_email(&self, job: &JobListing, to: &str) -> Result<Response, Box<dyn Error>> {
        let tos: Mailboxes = to.parse()?;
        let header: To = tos.into();
        let from = format!("upwork rss<{}>", self.smtp_username).parse()?;
        let body = job.description.to_owned();

        let email = Message::builder()
            .mailbox(header)
            .from(from)
            .subject(job.title.to_owned())
            .singlepart(SinglePart::html(body))?;

        let creds = Credentials::new(self.smtp_username.to_owned(), self.smtp_password.to_owned());

        let mailer = SmtpTransport::relay(&self.smtp_server)?
            .port(self.smtp_port)
            .credentials(creds)
            .build();


        return Ok(mailer.send(&email)?);
    }
}

