use crate::utils::file;
use async_trait::async_trait;
use sdk::error::Error;
use std::fmt::Debug;
use tokio::io::AsyncWriteExt;

#[async_trait]
pub trait Persister: Sync + Send {
    async fn append(&self, path: &str, bytes: &[u8]) -> Result<(), Error>;
    async fn overwrite(&self, path: &str, bytes: &[u8]) -> Result<(), Error>;
}

impl Debug for dyn Persister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Persister")
            .field("type", &"Persister")
            .finish()
    }
}

#[derive(Debug)]
pub struct FilePersister;

#[derive(Debug)]
pub struct FileWithSyncPersister;

unsafe impl Send for FilePersister {}
unsafe impl Sync for FilePersister {}

unsafe impl Send for FileWithSyncPersister {}
unsafe impl Sync for FileWithSyncPersister {}

#[async_trait]
impl Persister for FilePersister {
    async fn append(&self, path: &str, bytes: &[u8]) -> Result<(), Error> {
        let mut file = file::append(path).await?;
        file.write_all(bytes).await?;
        Ok(())
    }

    async fn overwrite(&self, path: &str, bytes: &[u8]) -> Result<(), Error> {
        let mut file = file::write(path).await?;
        file.write_all(bytes).await?;
        Ok(())
    }
}

#[async_trait]
impl Persister for FileWithSyncPersister {
    async fn append(&self, path: &str, bytes: &[u8]) -> Result<(), Error> {
        let mut file = file::append(path).await?;
        file.write_all(bytes).await?;
        file.sync_all().await?;
        Ok(())
    }

    async fn overwrite(&self, path: &str, bytes: &[u8]) -> Result<(), Error> {
        let mut file = file::write(path).await?;
        file.write_all(bytes).await?;
        file.sync_all().await?;
        Ok(())
    }
}
