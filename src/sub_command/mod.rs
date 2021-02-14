use anyhow::Result;
use clap::Clap;

use crate::context::Context;

mod login;
mod tweet;

#[derive(Debug, Clap)]
pub enum SubCommand {
    Login(login::Login),
    Tweet(tweet::Tweet),
}

impl SubCommand {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        match self {
            SubCommand::Login(login) => login.run(ctx).await?,
            SubCommand::Tweet(tweet) => tweet.run(ctx).await?,
        }

        Ok(())
    }
}
