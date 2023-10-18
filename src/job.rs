use std::{time, num, error};
use rss;
use reqwest;
use url;
use lru;
use regex;

#[derive(Debug)]
pub struct JobListing {
    pub title: String,
    pub description: String,
    pub country: String,
}

pub struct JobManager {
    query: String,
    paging: usize,
    seen_links: lru::LruCache<String, ()>,
    client: reqwest::Client,
    is_fresh: bool,
}

impl JobManager {
    pub fn new(query: &str, paging: usize) -> Result<Self, Box<dyn error::Error>> {
        let client = reqwest::Client::builder()
            .timeout(time::Duration::from_secs(15))
            .build()?;

        let cap = num::NonZeroUsize::new(1000 * paging)
            .ok_or("capacity must be a non-zero value")?;

        return Ok(Self {
            query: query.to_string(),
            paging: paging,
            seen_links: lru::LruCache::new(cap),
            client: client,
            is_fresh: true,
        });
    }

    fn build_url(&self) -> Result<String, url::ParseError> {
        let mut base_url = url::Url::parse("https://www.upwork.com/ab/feed/jobs/rss")?;
        base_url.query_pairs_mut()
            .append_pair("q", &self.query)
            .append_pair("sort", "recency")
            .append_pair("paging", &format!("0;{}", self.paging));
        Ok(base_url.to_string())
    }

    pub async fn fetch_new_jobs(&mut self) -> Result<Vec<JobListing>, Box<dyn error::Error>> {
        let url = self.build_url()?;
        let mut new_jobs = Vec::new();

        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await?;
        let channel = rss::Channel::read_from(text.as_bytes())?;

        for item in channel.items() {
            let link = item.link().unwrap_or_default().to_owned();
            if self.seen_links.put(link, ()).is_none() {
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


    fn parse_job(item: &rss::Item) -> Result<JobListing, Box<dyn error::Error>> {
        let desc = scraper::Html::parse_fragment(item.description().unwrap_or_default());

        let re = regex::Regex::new(r"<b>Country</b>:([^<]+)")?;
        let country = match re.captures(desc.html().as_str()) {
            Some(c) => { c.get(1).map_or("", |m| m.as_str()).trim().to_string() }
            None => String::new()
        };

        Ok(JobListing {
            title: item.title().unwrap_or_default().to_string(),
            description: desc.html(),
            country: country,
        })
    }
}


