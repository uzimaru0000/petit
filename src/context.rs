use anyhow::Result;
use chrono::{DateTime, Utc};
use kuon::{OAuthToken, TrimTweet, TwitterAPI};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufReader, BufWriter};

use crate::utils::stdio::read;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cache {
    pub latest_call: Option<DateTime<Utc>>,
    pub timeline: Vec<TrimTweet>,
    pub count: i32,
}

pub struct Context {
    pub client: Option<kuon::TwitterAPI>,
    pub api_key: String,
    pub api_secret: String,
    pub cache: Option<Cache>,
}

impl Context {
    pub async fn new() -> Result<Self> {
        let api_key = option_env!("API_KEY").unwrap();
        let api_secret = option_env!("API_SECRET_KEY").unwrap();

        let client = Self::build_client(api_key, api_secret).await;
        let cache = Self::get_cache().await;

        Ok(Self {
            client,
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
            cache,
        })
    }

    pub fn oauth_token_path() -> PathBuf {
        let home_dir = std::env::var("HOME").unwrap();
        let path = {
            let mut path = PathBuf::new();
            path.push(home_dir);
            path.push(".petit");
            path
        };
        path
    }

    async fn get_oauth_token() -> Option<OAuthToken> {
        let file = File::open(Self::oauth_token_path()).await.ok()?;
        let mut reader = BufReader::new(file);
        let json = read(&mut reader).await.ok()?;
        serde_json::from_str(&json).ok()?
    }

    fn cache_file_path() -> PathBuf {
        let home_dir = std::env::var("HOME").unwrap();
        let path = {
            let mut path = PathBuf::new();
            path.push(home_dir);
            path.push(".cache");
            path.push("petit");
            path
        };
        path
    }

    async fn get_cache() -> Option<Cache> {
        let file = File::open(Self::cache_file_path()).await.ok()?;
        let mut reader = BufReader::new(file);
        let json = read(&mut reader).await.ok()?;
        serde_json::from_str(&json).ok()?
    }

    pub async fn save_cache(cache: &Cache) -> Result<()> {
        let file = File::create(Context::cache_file_path()).await?;
        let mut writer = BufWriter::new(file);

        let data = serde_json::to_string(cache)?;
        writer.write_all(data.as_bytes()).await?;
        writer.flush().await?;

        Ok(())
    }

    async fn build_client(api_key: &str, api_secret: &str) -> Option<kuon::TwitterAPI> {
        let oauth = Self::get_oauth_token().await?;
        let client = TwitterAPI::builder()
            .api_key(api_key)
            .api_secret_key(api_secret)
            .access_token(oauth.token)
            .access_token_secret(oauth.secret)
            .build()
            .await
            .ok()?;

        Some(client)
    }
}
