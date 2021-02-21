use anyhow::Result;
use tokio::io::{AsyncReadExt, BufReader};

pub async fn read<R: AsyncReadExt + std::marker::Unpin>(
    reader: &mut BufReader<R>,
) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf).await?;
    Ok(buf)
}

pub fn read_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf)?;
    Ok(buf)
}
