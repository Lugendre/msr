use crate::domain::msr::{Album, AlbumDetail, AlbumSummary, MsrResponse, Song, SongSummaries};
use crate::domain::repository::msr_repository::UsesMsrRepository;
use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use url::Url;

#[derive(Debug, Clone)]
pub struct WebApiMsrRepository {
    client: reqwest::Client,
    base_url: Url,
}

impl WebApiMsrRepository {
    pub fn new(client: reqwest::Client, base_url: Url) -> Self {
        Self { client, base_url }
    }
}

async fn fetch_bytes(repo: &WebApiMsrRepository, url: Url) -> Result<Bytes> {
    let res = repo.client.get(url).send().await?.bytes().await?;
    Ok(res)
}

async fn fetch_json<U>(repo: &WebApiMsrRepository, url: Url) -> Result<U>
where
    U: serde::de::DeserializeOwned,
{
    let res = repo.client.get(url).send().await?.json().await?;
    Ok(res)
}

#[async_trait]
impl UsesMsrRepository for WebApiMsrRepository {
    async fn fetch_song(&self, song_id: String) -> Result<Song> {
        let url = self.base_url.join(format!("song/{}", song_id).as_str())?;
        let song = fetch_json::<MsrResponse<Song>>(self, url).await?;
        Ok(song.data)
    }

    async fn fetch_all_songs(&self) -> Result<SongSummaries> {
        let url = self.base_url.join("songs")?;
        let song_summaries = fetch_json::<MsrResponse<SongSummaries>>(self, url).await?;
        Ok(song_summaries.data)
    }

    async fn fetch_album(&self, album_id: String) -> Result<Album> {
        let url = self
            .base_url
            .join(format!("album/{}/data", album_id).as_str())?;
        let album = fetch_json::<MsrResponse<Album>>(self, url).await?;
        Ok(album.data)
    }

    async fn fetch_album_detail(&self, album_id: String) -> Result<AlbumDetail> {
        let url = self
            .base_url
            .join(format!("album/{}/detail", album_id).as_str())?;
        let album_detail = fetch_json::<MsrResponse<AlbumDetail>>(self, url).await?;
        Ok(album_detail.data)
    }

    async fn fetch_all_albums(&self) -> Result<Vec<AlbumSummary>> {
        let url = self.base_url.join("albums")?;
        let album_summaries = fetch_json::<MsrResponse<Vec<AlbumSummary>>>(self, url).await?;
        Ok(album_summaries.data)
    }

    async fn fetch_raw_song(&self, source_url: Url) -> Result<Bytes> {
        let raw_audio = fetch_bytes(self, source_url).await?;
        Ok(raw_audio)
    }

    async fn fetch_cover_image(&self, cover_url: Url) -> Result<Bytes> {
        let cover_image = fetch_bytes(self, cover_url).await?;
        Ok(cover_image)
    }
}
