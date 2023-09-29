use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use lettre::message::header::To;
use lettre::message::{Mailboxes, SinglePart};
use lettre::transport::smtp::response::Response;

use crate::job::JobListing;

pub struct EmailSender {
    client: SmtpTransport,
}

impl EmailSender {
    pub fn new(smtp_server: String, smtp_port: u16, smtp_username: String, smtp_password: String) -> Result<Self, Box<dyn std::error::Error>> {
        let creds = Credentials::new(smtp_username.clone(), smtp_password.clone());
        let client = SmtpTransport::relay(&smtp_server)?
            .port(smtp_port)
            .credentials(creds)
            .build();

        return Ok(Self { client });
    }

    pub fn send_email(&self, job: &JobListing, from: String, recipient: String) -> Result<Response, Box<dyn std::error::Error>> {
        let tos: Mailboxes = recipient.parse()?;
        let header: To = tos.into();
        let from = format!("upwork rss<{}>", from).parse()?;
        let body = job.description.clone();

        let email = Message::builder()
            .mailbox(header)
            .from(from)
            .subject(job.title.clone())
            .singlepart(SinglePart::html(body))?;


        Ok(self.client.send(&email)?)
    }
}
