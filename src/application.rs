use clap::Clap;

use crate::{context::Context, sub_command::SubCommand};
use anyhow::Result;

#[derive(Debug, Clap)]
#[clap(version = "0.0.2", author = "uzimaru0000<shuji365630@gmail.com>")]
pub struct Application {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

impl Application {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        self.subcmd.run(ctx).await
    }
}
