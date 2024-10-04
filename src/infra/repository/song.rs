use crate::domain::repository::song_repository::UsesSongRepository;
use crate::domain::song::{Album, AlbumId, AudioRawData, Song, SongId};
use crate::infra::resource::database::{query_all_and_values, query_one_and_values, read_only_transaction, ProvideDatabase};
use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use std::path::PathBuf;
use crate::errors::Error;
use crate::errors::infra::InfraError;

pub trait DatabaseSongRepository: ProvideDatabase + Send + Sync + 'static {}

#[async_trait]
impl<R: DatabaseSongRepository> UsesSongRepository for R {
    async fn get_song(&self, song_id: SongId) -> Result<Option<Song>> {
        let id: u32 = song_id.into();
        read_only_transaction(self, |tx| {
            Box::pin(async move {
                let song_query = match query_one_and_values(
                    tx,
                    r"SELECT id, name, track_number, disk_number, source_path, audio_formats.format AS format, album_id,
                            FROM songs
                                INNER JOIN audio_formats ON songs.audio_format_id = audio_formats.id
                            WHERE songs.id = ? AND songs.is_deleted = false
                            ",
                    vec![id.into()],
                )
                .await.map_err(Into::into)? {
                    Some(song_query) => song_query,
                    None => return Ok(None),
                };

                let raw = Bytes::new(); // TODO: ファイルシステムからデータを取得する
                let song_id_u32 = song_query.try_get::<u32>("", "id").map_err(Into::into)?;
                let song_id: SongId = song_id_u32.into();
                let album_id: AlbumId = song_query.try_get::<u32>("", "album_id").map_err(Into::into)?.into();
                // TODO: Domainに移動
                let audio_format = song_query.try_get::<String>("", "format").map_err(Into::into)?.parse().unwrap();
                let path = PathBuf::from(song_query.try_get::<String>("", "source_path").map_err(Into::into)?);
                let audio_raw_data = AudioRawData::reconstruct(raw, audio_format, path);

                let artists_query = query_all_and_values(
                    tx,
                    r"SELECT name FROM artists
                            WHERE artists.id = ? AND artists.is_deleted = false
                            ",
                    vec![song_id_u32.into()],
                ).await.map_err(Into::into)?;
                let artists = artists_query
                    .iter()
                    .map(|artist_query| artist_query.try_get("", "name").map_err(Into::into))
                    .collect::<std::result::Result<Vec<String>, _>>()?;

                let song = Song::try_reconstruct(
                    song_id,
                    song_query.try_get("", "name").map_err(Into::into)?,
                    album_id,
                    song_query.try_get("", "track_number").map_err(Into::into)?,
                    song_query.try_get("", "disk_number").map_err(Into::into)?,
                    audio_raw_data,
                    artists,
                )?;

                Ok::<_, Error>(Some(song))
            })
        })
        .await
        .map_err(Into::into)
    }

    async fn save_song(&self, song: Song) -> Result<()> {
        Ok(())
    }

    async fn delete_song(&self, song_id: SongId) -> Result<()> {
        Ok(())
    }

    async fn get_all_songs_id(&self) -> Result<Vec<SongId>> {
        Ok(vec![])
    }

    async fn get_all_song(&self) -> Result<Vec<Song>> {
        Ok(vec![])
    }

    async fn save_songs(&self, songs: Vec<Song>) -> Result<()> {
        Ok(())
    }

    async fn get_album(&self, album_id: AlbumId) -> Result<Option<Album>> {
        Ok(None)
    }

    async fn save_album(&self, album: Album) -> Result<()> {
        Ok(())
    }

    async fn delete_album(&self, album_id: AlbumId) -> Result<()> {
        Ok(())
    }

    async fn get_all_album(&self) -> Result<Vec<Album>> {
        Ok(vec![])
    }

    async fn save_all_album(&self, albums: Vec<Album>) -> Result<()> {
        Ok(())
    }
}
