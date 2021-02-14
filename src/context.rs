use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Result;
use kuon::{OAuthToken, TwitterAPI};

pub struct Context {
    pub client: Option<kuon::TwitterAPI>,
    pub api_key: String,
    pub api_secret: String,
}

impl Context {
    pub async fn new() -> Result<Self> {
        let api_key = std::env::var("API_KEY")?;
        let api_secret = std::env::var("API_SECRET_KEY")?;
        let client = Self::build_client(&api_key, &api_secret).await;

        Ok(Self {
            client,
            api_key,
            api_secret,
        })
    }

    pub fn oauth_token_path() -> PathBuf {
        let home_dir = std::env::var("HOME").unwrap();
        let path = {
            let mut path = PathBuf::new();
            path.push(home_dir);
            path.push(".kuon");
            path
        };
        path
    }

    async fn build_client(api_key: &str, api_secret: &str) -> Option<kuon::TwitterAPI> {
        let file = File::open(Self::oauth_token_path()).ok()?;
        let reader = BufReader::new(file);
        let oauth_token: OAuthToken = serde_json::from_reader(reader).ok()?;
        let client = TwitterAPI::builder()
            .api_key(api_key)
            .api_secret_key(api_secret)
            .access_token(oauth_token.token)
            .access_token_secret(oauth_token.secret)
            .build()
            .await
            .ok()?;

        Some(client)
    }
}
