use anyhow::{Context as _, Result};
use clap::Clap;
use tokio::io::{stdout, AsyncWriteExt, BufWriter};

use crate::context::Context;

#[derive(Debug, Clap)]
pub struct Tweet {
    content: String,
}

impl Tweet {
    pub async fn run(&self, ctx: Context) -> Result<()> {
        let mut stdout = BufWriter::new(stdout());

        let client = ctx
            .client
            .with_context(|| "Please login. run \"kuon login\"")?;

        client.tweet(&self.content).await?;
        stdout.write_all(b"success!").await?;

        Ok(())
    }
}
