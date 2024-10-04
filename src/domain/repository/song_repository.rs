use crate::domain::song::*;
use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait UsesSongRepository: Send + Sync + 'static {
    async fn get_song(&self, song_id: SongId) -> Result<Option<Song>>;
    async fn save_song(&self, song: Song) -> Result<()>;
    async fn delete_song(&self, song_id: SongId) -> Result<()>;
    async fn get_all_songs_id(&self) -> Result<Vec<SongId>>;
    async fn get_all_song(&self) -> Result<Vec<Song>>;
    async fn save_songs(&self, songs: Vec<Song>) -> Result<()>;
    async fn get_album(&self, album_id: AlbumId) -> Result<Option<Album>>;
    async fn save_album(&self, album: Album) -> Result<()>;
    async fn delete_album(&self, album_id: AlbumId) -> Result<()>;
    async fn get_all_album(&self) -> Result<Vec<Album>>;
    async fn save_all_album(&self, albums: Vec<Album>) -> Result<()>;
}

pub trait ProvideSongRepository {
    type SongRepository: UsesSongRepository + Send + Sync + 'static;
    fn provide_song_repository(&self) -> &Self::SongRepository;
}
