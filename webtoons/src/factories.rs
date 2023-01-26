use anyhow::{bail, Result};
use rand::Rng;
use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};

pub struct BlockingReferClientFactory;

impl BlockingReferClientFactory {
    /// # Errors
    ///
    /// Will panic if there it can't connect to URL.
    pub fn get(url: &str) -> Result<reqwest::blocking::Response> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
            .build()?;
        // let mut rng = rand::thread_rng();

        let mut retries = 5;
        let mut wait = 1;
        //
        // let stop_rng = rng.gen_range(1..3);
        //
        // thread::sleep(Duration::from_secs(stop_rng));

        let response = loop {
            info!("Making request to {url}");
            match client
                .get(url)
                .header("referer", "https://www.webtoons.com/")
                .send()
            {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        warn!("Retrying connection to {url}\nRetries left: {retries}\nWait time: {wait}");
                        thread::sleep(Duration::from_secs(wait));
                        wait *= 2;
                    } else {
                        error!("Out of retires, failed to connect to {url}");
                        bail!("Cannot connect. Check URL: {url}");
                    }
                }
                Ok(ok) => break ok,
            }
        };

        Ok(response)
    }
}
