use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use crate::{context::Context, utils::read_line, utils::write};
use anyhow::Result;
use clap::Clap;
use kuon::{Callback, OAuthRequestToken, OAuthToken, TwitterAPI};

#[derive(Debug, Clap)]
pub struct Login {}

impl Login {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        if let Some(_) = ctx.client {
            println!("Already logged in!!");
            return Ok(());
        }

        let builder = TwitterAPI::builder()
            .api_key(ctx.api_key)
            .api_secret_key(ctx.api_secret);
        let request_token = builder.pre_build(Callback::PIN).await?;

        println!("Please access {} and login", oauth_url(&request_token));
        println!("---input pin code---");

        let pin = Self::input_pin()?;
        let api = builder.build(request_token, &pin).await?;

        let oauth_token = api.oauth_token();
        Self::write_token(&oauth_token)?;

        Ok(())
    }

    fn input_pin() -> Result<String> {
        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin);
        let pin = read_line(&mut reader)?;
        Ok(pin)
    }

    fn write_token(oauth_token: &OAuthToken) -> Result<()> {
        let serialized = serde_json::to_string(oauth_token)?;
        let file = File::create(Context::oauth_token_path())?;
        let mut writer = BufWriter::new(file);
        write(&mut writer, serialized.as_bytes())?;

        Ok(())
    }
}

fn oauth_url(request_token: &OAuthRequestToken) -> String {
    format!(
        "https://api.twitter.com/oauth/authorize?oauth_token={}",
        request_token.token
    )
}
