use std::collections::{HashSet, VecDeque};
use std::io::Cursor;
use rss::{Channel, Item};
use reqwest;
use url::Url;

#[derive(Debug)]
pub struct JobListing {
    pub title: String,
    pub description: String,
}

pub struct JobManager {
    query: String,
    paging: usize,
    seen_links: HashSet<String>,
    link_order: VecDeque<String>,
}

impl JobManager {
    pub fn new(query: &str, paging: usize) -> Self {
        Self {
            query: query.to_string(),
            paging,
            seen_links: HashSet::new(),
            link_order: VecDeque::new(),
        }
    }

    fn build_url(&self) -> Result<String, url::ParseError> {
        let mut base_url = Url::parse("https://www.upwork.com/ab/feed/jobs/rss")?;
        base_url.query_pairs_mut()
            .append_pair("q", &self.query)
            .append_pair("sort", "recency")
            .append_pair("paging", &format!("0;{}", self.paging));
        Ok(base_url.to_string())
    }

    pub fn fetch_new_jobs(&mut self) -> Result<Vec<JobListing>, Box<dyn std::error::Error>> {
        let url = self.build_url()?;
        let mut new_jobs = Vec::new();
        const MAX_CAPACITY: usize = 100;

        if let Ok(resp) = reqwest::blocking::get(&url) {
            if let Ok(text) = resp.text() {
                if let Ok(channel) = Channel::read_from(text.as_bytes()) {
                    for item in channel.items() {
                        let link = item.link().unwrap_or_default().to_string();
                        if self.seen_links.insert(link.clone()) {
                            self.link_order.push_back(link);
                            if self.seen_links.len() > MAX_CAPACITY {
                                if let Some(oldest_link) = self.link_order.pop_front() {
                                    self.seen_links.remove(&oldest_link);
                                }
                            }
                            let job = Self::parse_job(item)?;
                            new_jobs.push(job);
                        }
                    }
                }
            }
        }
        Ok(new_jobs)
    }

    pub fn display(&self, job: &JobListing) {
        const DEFAULT_WIDTH: usize = 100;
        let body = html2text::from_read(Cursor::new(job.description.clone()), DEFAULT_WIDTH);
        println!("Title: {}", job.title);
        println!("Description: {}", body);
        println!("{}", "-".repeat(DEFAULT_WIDTH));
    }

    fn parse_job(item: &Item) -> Result<JobListing, Box<dyn std::error::Error>> {
        let desc = scraper::Html::parse_fragment(item.description().unwrap_or_default());
        Ok(JobListing {
            title: item.title().unwrap_or_default().to_string(),
            description: desc.html(),
        })
    }
}


