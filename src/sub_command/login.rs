use crate::{context::Context, utils::stdio::read_stdin};
use anyhow::Result;
use clap::Clap;
use kuon::{Callback, OAuthRequestToken, OAuthToken, TwitterAPI};
use tokio::{
    fs::File,
    io::{stdout, AsyncWriteExt, BufWriter},
};

#[derive(Debug, Clap)]
pub struct Login {}

impl Login {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        let mut stdout = BufWriter::new(stdout());

        if ctx.client.is_some() {
            stdout.write_all(b"Already logged in!!").await?;
            stdout.flush().await?;
            return Ok(());
        }

        let builder = TwitterAPI::builder()
            .api_key(ctx.api_key.clone())
            .api_secret_key(ctx.api_secret.clone());
        let request_token = builder.pre_build(Callback::Pin).await?;

        stdout
            .write_all(
                format!("Please access {} and login\n", oauth_url(&request_token)).as_bytes(),
            )
            .await?;
        stdout.write_all(b"Input pin code : ").await?;
        stdout.flush().await?;

        let pin = read_stdin()?;
        let api = builder.build(request_token, &pin).await?;

        let oauth_token = api.oauth_token();
        Self::write_token(&oauth_token).await?;

        stdout.write_all(b"Logged in successfully!").await?;
        stdout.flush().await?;

        Ok(())
    }

    async fn write_token(oauth_token: &OAuthToken) -> Result<()> {
        let serialized = serde_json::to_string(oauth_token)?;
        let file = File::create(Context::oauth_token_path()).await?;
        let mut writer = BufWriter::new(file);
        writer.write_all(serialized.as_bytes()).await?;
        writer.flush().await?;

        Ok(())
    }
}

fn oauth_url(request_token: &OAuthRequestToken) -> String {
    format!(
        "https://api.twitter.com/oauth/authorize?oauth_token={}",
        request_token.token
    )
}
