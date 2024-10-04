use thiserror::Error;
use crate::domain::song::{AlbumId, SongId};

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("Failed to parse song id: {s}")]
    FailedToParseSongId {s: String},
    #[error("Failed to parse album id: {s}")]
    FailedToParseAlbumId {s: String},
    #[error("Failed to create audio raw data")]
    FailedToCreateAudioRawData,
    #[error("Failed to create song")]
    FailedToCreateSong {id: SongId},
    #[error("Failed to create album")]
    FailedToCreateAlbum {id: AlbumId},
    #[error("Failed to parse {s}")]
    FailedToParseOriginGame{s: String},
    #[error("Failed to get song in the album")]
    FailedToGetSongInAlbum {song_id: SongId, album_id: AlbumId},
}