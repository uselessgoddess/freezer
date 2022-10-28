use crate::Result;
use bytes::Bytes;
use format_bytes::format_bytes;
use redis::{aio::Connection, AsyncCommands};

pub struct ImageStore(Connection);

impl ImageStore {
    pub fn new(client: Connection) -> Self {
        Self(client)
    }

    fn image_path(key: &[u8]) -> Vec<u8> {
        format_bytes!(b"freezers:{}:image", key)
    }

    pub async fn image(&mut self, uuid: &[u8]) -> Result<Option<Bytes>> {
        self.0
            .get(&Self::image_path(uuid))
            .await
            .map_err(Into::into)
    }

    pub async fn load(&mut self, uuid: &[u8], bytes: &[u8]) -> Result<()> {
        self.0
            .set(&Self::image_path(uuid), bytes)
            .await
            .map_err(Into::into)
    }

    pub async fn remove(&mut self, uuid: &[u8]) -> Result<()> {
        self.0
            .del(&Self::image_path(uuid))
            .await
            .map_err(Into::into)
    }
}
