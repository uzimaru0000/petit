use anyhow::Result;
use clap::Clap;

use crate::context::Context;

mod login;
mod search;
mod timeline;
mod tweet;

#[derive(Debug, Clap)]
pub enum SubCommand {
    Login(login::Login),
    Tweet(tweet::Tweet),
    #[clap(name = "tl")]
    TimeLine(timeline::TimeLine),
    Search(search::Search),
}

impl SubCommand {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        match self {
            SubCommand::Login(login) => login.run(ctx).await?,
            SubCommand::Tweet(tweet) => tweet.run(ctx).await?,
            SubCommand::TimeLine(tl) => tl.run(ctx).await?,
            SubCommand::Search(search) => search.run(ctx).await?,
        }

        Ok(())
    }
}
