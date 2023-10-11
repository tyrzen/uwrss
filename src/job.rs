use std::io::Cursor;
use std::time::Duration;
use rss::{Channel, Item};
use reqwest;
use reqwest::Client;
use url::Url;
use lru::LruCache;
use std::num::NonZeroUsize;
use regex::{Captures, Regex};

#[derive(Debug)]
pub struct JobListing {
    pub title: String,
    pub description: String,
    pub country: String,
}

impl JobListing {
    pub fn display(&self) {
        const RENDERING_WIDTH: usize = 100;
        let body = html2text::from_read(Cursor::new(self.description.clone()), RENDERING_WIDTH);
        println!("Title: {}", self.title);
        println!("Description: {}", body);
        println!("{}", "-".repeat(RENDERING_WIDTH));
    }
}

pub struct JobManager {
    query: String,
    paging: usize,
    seen_links: LruCache<String, ()>,
    client: Client,
    is_fresh: bool,
}

impl JobManager {
    pub fn new(query: &str, paging: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()?;

        let cap = match NonZeroUsize::new(1000 * paging) {
            Some(cap) => cap,
            None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "capacity must be a non-zero value"))),
        };

        return Ok(Self {
            query: query.to_string(),
            paging,
            seen_links: LruCache::new(cap),
            client,
            is_fresh: true,
        });
    }

    fn build_url(&self) -> Result<String, url::ParseError> {
        let mut base_url = Url::parse("https://www.upwork.com/ab/feed/jobs/rss")?;
        base_url.query_pairs_mut()
            .append_pair("q", &self.query)
            .append_pair("sort", "recency")
            .append_pair("paging", &format!("0;{}", self.paging));
        Ok(base_url.to_string())
    }

    pub async fn fetch_new_jobs(&mut self) -> Result<Vec<JobListing>, Box<dyn std::error::Error>> {
        let url = self.build_url()?;
        let mut new_jobs = Vec::new();

        let resp = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(err) => return Err(Box::new(err)),
        };

        let text = match resp.text().await {
            Ok(text) => text,
            Err(err) => return Err(Box::new(err)),
        };

        let channel = match Channel::read_from(text.as_bytes()) {
            Ok(ch) => ch,
            Err(err) => return Err(Box::new(err)),
        };

        for item in channel.items() {
            let link = item.link().unwrap_or_default().to_owned();
            let title = item.title().unwrap_or_default().to_owned();
            if self.seen_links.put(title.clone(), ()).is_none() {
                println!("{}: {}", title, link);
                let job = Self::parse_job(item)?;
                new_jobs.push(job);
            }
        }

        if self.is_fresh {
            self.is_fresh = false;
            return Ok(Default::default());
        }

        return Ok(new_jobs);
    }


    fn parse_job(item: &Item) -> Result<JobListing, Box<dyn std::error::Error>> {
        let desc = scraper::Html::parse_fragment(item.description().unwrap_or_default());

        let re = Regex::new(r"<b>Country</b>:([^<]+)")?;
        let country = match re.captures(desc.html().as_str()) {
            Some(c) => { c.get(1).map_or("", |m| m.as_str()).trim().to_string() }
            None => String::new()
        };

        Ok(JobListing {
            title: item.title().unwrap_or_default().to_string(),
            description: desc.html(),
            country,
        })
    }
}


