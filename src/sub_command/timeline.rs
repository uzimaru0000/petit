use std::collections::HashMap;

use crate::context::Context;
use anyhow::{Context as _, Result};
use clap::Clap;
use tokio::io::{stdout, AsyncWriteExt, BufWriter};

const ENDPOINT: &str = "https://api.twitter.com/1.1/statuses/home_timeline.json";

#[derive(Debug, Clap)]
#[clap(name = "tl")]
pub struct TimeLine {
    #[clap(short, long)]
    stream: bool,
}

impl TimeLine {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        let client = ctx
            .client
            .with_context(|| "Please login. run \"kuon login\"")?;

        let mut stdout = BufWriter::new(stdout());

        let tweet_list: Vec<kuon::Tweet> = client.raw_get(ENDPOINT, &HashMap::new(), None).await?;

        for tweet in tweet_list {
            let tweet = format!(
                "{} @{} | {}\n",
                tweet.user.name, tweet.user.screen_name, tweet.text
            );
            stdout.write_all(tweet.as_bytes()).await?;
        }
        stdout.flush().await?;

        Ok(())
    }
}
