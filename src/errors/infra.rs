use std::path::PathBuf;
use thiserror::Error;
use crate::domain::song::{AlbumId, SongId};

#[derive(Debug, Error, PartialEq)]
pub enum InfraError {
    #[error("Database error: {source}")]
    Database {
        #[from]
        source: sea_orm::DbErr,
    },
    #[error("Transaction error: {cause}")]
    DatabaseTransaction { cause: String },
    #[error("Song not found: {id}")]
    SongNotFound { id: SongId },
    #[error("Album not found: {id}")]
    AlbumNotFound { id: AlbumId },
    #[error("Failed to load audio raw data: {path}")]
    FailedToLoadAudioRawData {path: PathBuf},
}

impl<E> From<sea_orm::TransactionError<E>> for InfraError
where
    E: std::error::Error,
{
    fn from(err: sea_orm::TransactionError<E>) -> Self {
        InfraError::DatabaseTransaction {
            cause: err.to_string(),
        }
    }
}