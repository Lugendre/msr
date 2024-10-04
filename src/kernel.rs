use std::sync::Arc;
use crate::domain::repository::msr_repository::ProvideMsrRepository;
use crate::domain::repository::song_repository::{ProvideSongRepository, UsesSongRepository};
use crate::infra::repository::msr::WebApiMsrRepository;
use crate::infra::repository::song::DatabaseSongRepository;
use crate::usecase::add_new_song::AddNewSongUseCase;
use anyhow::Result;
use reqwest::header::HeaderMap;
use sea_orm::DatabaseConnection;
use url::Url;
use crate::infra::resource::database::ProvideDatabase;

#[derive(Debug, Clone)]
pub struct Kernel {
    msr_repository: WebApiMsrRepository,
    song_repository: SongRepositoryImpl,
}

// TODO: もっとふさわしい名前があるはず
#[derive(Debug, Clone)]
pub struct SongRepositoryImpl {
    pub db_connection: Arc<DatabaseConnection>,
}

impl ProvideDatabase for SongRepositoryImpl {
    fn provide_database(&self) -> &DatabaseConnection {
        &self.db_connection
    }
}

impl Kernel {
    pub fn try_new(header_map: HeaderMap) -> Result<Self> {
        let client = reqwest::Client::builder()
            .default_headers(header_map)
            .build()?;
        let db_connection = Arc::new(todo!());
        Ok(Self {
            msr_repository: WebApiMsrRepository::new(
                client.clone(),
                Url::parse("https://monster-siren.hypergryph.com/api/")?,
            ),
            song_repository: SongRepositoryImpl{db_connection},
        })
    }
}

impl ProvideMsrRepository for Kernel {
    type MsrRepository = WebApiMsrRepository;
    fn provide_msr_repository(&self) -> &Self::MsrRepository {
        &self.msr_repository
    }
}

impl ProvideDatabase for Kernel {
    fn provide_database(&self) -> &DatabaseConnection {
        &self.song_repository.provide_database()
    }
}

impl ProvideSongRepository for Kernel {
    type SongRepository = SongRepositoryImpl;
    fn provide_song_repository(&self) -> &Self::SongRepository {
        &self.song_repository
    }
}
impl DatabaseSongRepository for SongRepositoryImpl {}
impl DatabaseSongRepository for Kernel {}

impl AddNewSongUseCase for Kernel {}
