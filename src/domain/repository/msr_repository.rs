use crate::domain::msr::*;
use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use mockall::automock;
use url::Url;

#[automock]
#[async_trait]
pub trait UsesMsrRepository: Send + Sync + 'static {
    async fn fetch_song(&self, song_id: String) -> Result<Song>;
    async fn fetch_all_songs(&self) -> Result<SongSummaries>;
    async fn fetch_album(&self, album_id: String) -> Result<Album>;
    async fn fetch_album_detail(&self, album_id: String) -> Result<AlbumDetail>;
    async fn fetch_all_albums(&self) -> Result<Vec<AlbumSummary>>;
    async fn fetch_raw_song(&self, source_url: Url) -> Result<Bytes>;
    async fn fetch_cover_image(&self, cover_url: Url) -> Result<Bytes>;
}

pub trait ProvideMsrRepository {
    type MsrRepository: UsesMsrRepository + Send + Sync + 'static;
    fn provide_msr_repository(&self) -> &Self::MsrRepository;
}

