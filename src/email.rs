use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use lettre::message::header::To;
use lettre::message::{Mailboxes, SinglePart};
use lettre::transport::smtp::response::Response;
use std::error::Error;

use crate::config::Config;
use crate::job::JobListing;

pub struct EmailSender {
    smtp_server: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
}

impl EmailSender {
    pub fn new(config: &Config) -> Self {
        Self {
            smtp_server: config.smtp_server.clone(),
            smtp_port: config.smtp_port,
            smtp_username: config.smtp_username.clone(),
            smtp_password: config.smtp_password.clone(),
        }
    }

    pub fn send_email(&self, job: &JobListing, recipient: String) -> Result<Response, Box<dyn Error>> {
        let tos: Mailboxes = recipient.parse()?;
        let header: To = tos.into();
        let from = format!("upwork rss<{}>", self.smtp_username).parse()?;
        let body = job.description.clone();

        let email = Message::builder()
            .mailbox(header)
            .from(from)
            .subject(job.title.clone())
            .singlepart(SinglePart::html(body))?;

        let creds = Credentials::new(self.smtp_username.clone(), self.smtp_password.clone());

        let mailer = SmtpTransport::relay(&self.smtp_server)?
            .port(self.smtp_port)
            .credentials(creds)
            .build();

        Ok(mailer.send(&email)?)
    }
}
