use std::error;
use lettre::{Transport, message::{self, header}, transport::smtp::{response, authentication}};
use crate::job;

pub struct EmailSender {
    client: lettre::SmtpTransport,
}

impl EmailSender {
    pub fn new(smtp_server: String, smtp_port: u16, smtp_username: String, smtp_password: String) -> Result<Self, Box<dyn error::Error>> {
        let creds = authentication::Credentials::new(smtp_username.clone(), smtp_password.clone());
        let client = lettre::SmtpTransport::relay(&smtp_server)?
            .port(smtp_port)
            .credentials(creds)
            .build();

        return Ok(Self { client });
    }

    pub fn send_email(&self, job: &job::JobListing, from: &str, recipient: &str) -> Result<response::Response, Box<dyn error::Error>> {
        let tos: message::Mailboxes = recipient.parse()?;
        let header: header::To = tos.into();
        let from = format!("upwork rss<{}>", from).parse()?;
        let body = job.description.clone();

        let email = lettre::Message::builder()
            .mailbox(header)
            .from(from)
            .subject(job.title.clone())
            .singlepart(message::SinglePart::html(body))?;

        Ok(self.client.send(&email)?)
    }
}
