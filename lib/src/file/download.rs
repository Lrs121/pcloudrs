use super::FileIdentifier;
use crate::error::Error;
use crate::http::HttpClient;
use crate::prelude::Command;
use std::io::Write;

#[derive(Debug)]
pub struct FileDownloadCommand<W> {
    identifier: FileIdentifier,
    writer: W,
}

impl<W: Write> FileDownloadCommand<W> {
    pub fn new(identifier: FileIdentifier, writer: W) -> Self {
        Self { identifier, writer }
    }
}

#[async_trait::async_trait(?Send)]
impl<W: Write> Command for FileDownloadCommand<W> {
    type Output = usize;
    type Error = Error;

    async fn execute(mut self, client: &HttpClient) -> Result<Self::Output, Self::Error> {
        let link = client.get_link_file(self.identifier).await?;
        let mut req = client.client.get(&link).send().await?;
        let mut size = 0;
        while let Some(chunk) = req.chunk().await? {
            size += self.writer.write(chunk.as_ref()).map_err(Error::Download)?;
        }
        Ok(size)
    }
}
