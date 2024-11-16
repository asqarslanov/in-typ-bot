use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

pub trait AsyncWriteln {
    async fn writeln(&mut self, contents: &[u8]) -> io::Result<()>;
}

impl AsyncWriteln for File {
    async fn writeln(&mut self, contents: &[u8]) -> io::Result<()> {
        self.write_all(contents).await?;
        self.write_all(b"\n").await?;
        Ok(())
    }
}
