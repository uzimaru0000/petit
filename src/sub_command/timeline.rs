use std::str::FromStr;

use crate::component::tweet::TweetView;
use crate::context::{Cache, Context};
use anyhow::{Context as _, Result};
use chrono::Utc;
use clap::Clap;
use colored::{self, Colorize};
use kuon::{TrimTweet, TwitterAPI};
use tokio::io::{stdout, AsyncWriteExt, BufWriter, Stdout};

#[derive(Debug)]
enum DisplayType {
    Standard,
    Json,
    Csv,
}

impl FromStr for DisplayType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(Self::Standard),
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            _ => Err(String::from("no match string")),
        }
    }
}

#[derive(Debug, Clap)]
#[clap(name = "tl")]
pub struct TimeLine {
    #[clap(long, short, default_value = "standard")]
    display: DisplayType,
    id: Option<String>,
}

impl TimeLine {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        let client = ctx
            .client
            .with_context(|| "Please login. run \"petit login\"")?;
        let tweet_list = if self.id.is_none()
            && ctx
                .cache
                .as_ref()
                .and_then(|x| x.latest_call)
                .map(|x| Self::is_latest_request(x))
                .unwrap_or(false)
        {
            ctx.cache
                .as_ref()
                .map(|x| x.timeline.clone())
                .unwrap_or_default()
        } else {
            Self::get_tweet(&client, self.id.clone()).await?
        };
        let mut stdout = BufWriter::new(stdout());
        Self::output(&mut stdout, &tweet_list, &self.display).await?;

        Ok(())
    }

    async fn get_tweet(client: &TwitterAPI, since_id: Option<String>) -> Result<Vec<TrimTweet>> {
        let mut timeline = client.home_timeline();
        if let Some(id) = since_id.and_then(|x| x.parse::<u64>().ok()) {
            timeline.since_id(id);
        }
        let tweet_list = timeline.count(30).send().await?;

        let now = chrono::Utc::now();
        let cache = Cache {
            latest_call: Some(now),
            timeline: tweet_list.clone(),
            count: 0,
        };
        Context::save_cache(&cache).await?;

        Ok(tweet_list)
    }

    async fn output(
        stdout: &mut BufWriter<Stdout>,
        tweet_list: &Vec<TrimTweet>,
        display: &DisplayType,
    ) -> Result<()> {
        match display {
            &DisplayType::Standard => {
                let output_line = tweet_list.iter().map(|x| TweetView::from(x)).map(|x| {
                    format!(
                        "{} {} {}\n {}\n\n",
                        x.user_name,
                        format!("@{}", x.screen_name).bright_red(),
                        x.retweet_user_name
                            .map(|x| format!("RT:@{}", x))
                            .unwrap_or_default()
                            .bright_green(),
                        x.tweet,
                    )
                });
                for line in output_line {
                    stdout.write_all(line.as_bytes()).await?;
                }
            }
            &DisplayType::Json => {
                let json = serde_json::to_string(&tweet_list)?;
                stdout.write_all(json.as_bytes()).await?;
            }
            &DisplayType::Csv => {
                let output_line = tweet_list.iter().map(|x| TweetView::from(x)).map(|x| {
                    format!(
                        "{}\t{}\t{}\t{}{}\n",
                        x.id,
                        x.user_name,
                        x.screen_name,
                        x.tweet.replace("\n", " "),
                        x.retweet_user_name
                            .as_ref()
                            .map(|x| String::from("\t") + x)
                            .unwrap_or_default()
                    )
                });
                for line in output_line {
                    stdout.write_all(line.as_bytes()).await?;
                }
            }
        }

        stdout.flush().await.with_context(|| "Output Error")
    }

    fn is_latest_request(latest_call: chrono::DateTime<Utc>) -> bool {
        let now = Utc::now();
        let delta = now - latest_call;

        delta.num_minutes() < 1
    }
}
