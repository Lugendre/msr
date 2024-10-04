use crate::domain::repository::msr_repository::{ProvideMsrRepository, UsesMsrRepository};
use crate::domain::repository::song_repository::{ProvideSongRepository, UsesSongRepository};
use crate::domain::song::{self, AlbumId, AudioRawData, OriginGame, SongId};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use indexmap::IndexSet;
use std::vec;
use crate::errors::domain::DomainError;

/// サイトの更新を見て新しい楽曲を登録するユースケース
/// Cloneのコストが小さい型に実装することを想定している。
#[async_trait]
pub trait UsesAddNewSongUseCase {
    /// MSRから最新の楽曲情報を取得し、DBと比較して新しい楽曲があれば登録する
    async fn add_new_songs(&self) -> Result<Vec<SongId>>;
}

/// [`UsesAddNewSongUseCase`]に必要な依存
pub trait AddNewSongUseCase:
    ProvideSongRepository + ProvideMsrRepository + Send + Sync + 'static + Clone
{
}

#[async_trait]
impl<R: AddNewSongUseCase> UsesAddNewSongUseCase for R {
    async fn add_new_songs(&self) -> Result<Vec<SongId>> {
        // MSRからすべての楽曲情報のIDを取得
        let msr_song_summaries = self
            .provide_msr_repository()
            .fetch_all_songs()
            .await?
            .list
            .into_iter()
            .map(|x| SongId::try_new(x.id))
            .collect::<Result<Vec<_>, DomainError>>()?;

        // DBからすべての楽曲情報のIDを取得
        let store_song_summaries = self
            .provide_song_repository()
            .get_all_songs_id()
            .await?
            .into_iter()
            .collect::<IndexSet<_>>();

        // 差分のIDを取得
        let new_songs_id = msr_song_summaries
            .into_iter()
            .filter(|x| store_song_summaries.contains(x))
            .collect::<Vec<_>>();
        // 新しい楽曲がなければ何もしない
        if new_songs_id.is_empty() {
            return Ok(vec![]);
        }

        fetch_and_save_songs(self, &new_songs_id).await?;

        Ok(new_songs_id)
    }
}

/// 新しい楽曲を取得し、保存する
/// * repositories: リポジトリ。Cloneのコストが小さいことを期待している。
/// * song_ids: 楽曲IDのリスト。MSRが提供するIDを期待している。
pub async fn fetch_and_save_songs<R: AddNewSongUseCase>(
    repositories: &R,
    song_ids: &[SongId],
) -> Result<()> {
    futures::stream::iter(song_ids.iter().copied())
        // Note: try_for_each_concurrentを使うためにResultで包んでいる
        .map(Ok)
        .try_for_each_concurrent(8, |song_id| {
            let repositories = repositories.clone();
            async move {
                let song = fetch_and_create_song(&repositories, song_id).await?;
                repositories
                    .provide_song_repository()
                    .save_song(song)
                    .await?;
                Ok(())
            }
        })
        .await?;
    Ok(())
}

/// 楽曲情報とアルバム情報を取得し、[`Song`]を作成する。
/// * repositories: リポジトリ。Cloneのコストが小さいことを期待している。
/// * song_id: 楽曲ID。MSRが提供するIDを期待している。
pub async fn fetch_and_create_song<R: AddNewSongUseCase>(
    repositories: &R,
    song_id: SongId,
) -> Result<song::Song> {
    let msr_song = repositories
        .provide_msr_repository()
        .fetch_song(format!("{song_id:0>6}"))
        .await?;
    let belong_album_id = AlbumId::try_new(msr_song.belong_album_id.clone())?;
    // Repositoryにアルバムが存在するか確認
    let song_list = if let Some(album) = repositories
        .provide_song_repository()
        .get_album(belong_album_id)
        .await?
    {
        album.song_list
    } else {
        // なければMSRから取得して登録
        let msr_album = repositories
            .provide_msr_repository()
            .fetch_album(msr_song.belong_album_id)
            .await?;
        let song_list = repositories
            .provide_msr_repository()
            .fetch_album_detail(format!("{belong_album_id:0>4}"))
            .await?
            .song_list
            .into_iter()
            .map(|raw| SongId::try_new(raw.id))
            .collect::<Result<Vec<_>, DomainError>>()?;
        let cover_image = repositories
            .provide_msr_repository()
            .fetch_cover_image(msr_album.cover_url)
            .await?;
        let album = song::Album::try_new(
            belong_album_id,
            msr_album.name,
            msr_album.intro,
            OriginGame::try_new(msr_album.belong)?,
            cover_image,
            msr_album.artists,
            song_list.clone(),
        )?;
        repositories
            .provide_song_repository()
            .save_album(album)
            .await?;
        song_list
    };

    let source = AudioRawData::try_new(
        repositories
            .provide_msr_repository()
            .fetch_raw_song(msr_song.source_url)
            .await?,
    )?;

    let song = song::Song::try_new(
        song_id,
        msr_song.name,
        belong_album_id,
        &song_list,
        source,
        msr_song.artists,
    )?;

    Ok(song)
}

#[cfg(test)]
mod tests {
    use crate::domain::msr::*;
    use crate::domain::repository::msr_repository::MockUsesMsrRepository;
    use crate::domain::repository::song_repository::MockUsesSongRepository;
    use bytes::Bytes;
    use std::sync::Arc;
    use url::Url;

    #[tokio::test]
    async fn test_add_new_songs() {
        #[derive(Clone)]
        struct Mock {
            song: Arc<MockUsesSongRepository>,
            msr: Arc<MockUsesMsrRepository>,
        }
        let mut song_mock = MockUsesSongRepository::new();
        song_mock.expect_get_all_songs_id().returning(|| Ok(vec![]));
        song_mock.expect_save_songs().returning(|_| Ok(()));
        song_mock.expect_get_album().returning(|_| Ok(None));
        song_mock.expect_save_song().returning(|_| Ok(()));
        song_mock.expect_save_album().returning(|_| Ok(()));

        let mut msr_mock = MockUsesMsrRepository::new();
        msr_mock.expect_fetch_all_songs().returning(|| {
            let list = SongSummaries {
                list: vec![SongSummary {
                    id: "000001".into(),
                    name: "song".into(),
                    belong_album_id: "0001".into(),
                    artists: vec!["artist".into()],
                }],
            };
            Ok(list)
        });
        msr_mock.expect_fetch_song().returning(|_| {
            let song = Song {
                id: "000001".into(),
                name: "song".into(),
                belong_album_id: "0001".into(),
                source_url: Url::parse("https://example.com").unwrap(),
                artists: vec!["artist".into()],
                lyric_url: None,
            };
            Ok(song)
        });
        msr_mock.expect_fetch_album().returning(|_| {
            let album = Album {
                id: "0001".into(),
                name: "album".into(),
                intro: "intro".into(),
                belong: "arknights".into(),
                cover_url: Url::parse("https://example.com").unwrap(),
                cover_de_url: Url::parse("https://example.com").unwrap(),
                artists: vec!["artist".into()],
            };
            Ok(album)
        });
        msr_mock.expect_fetch_album_detail().returning(|_| {
            let detail = AlbumDetail {
                id: "0001".into(),
                name: "album".into(),
                intro: "intro".into(),
                belong: "arknights".into(),
                cover_url: Url::parse("https://example.com").unwrap(),
                cover_de_url: Url::parse("https://example.com").unwrap(),
                song_list: vec![SongShort {
                    id: "000001".into(),
                    name: "song".into(),
                    artists: vec!["artist".into()],
                }],
            };
            Ok(detail)
        });
        msr_mock
            .expect_fetch_cover_image()
            .returning(|_| Ok(Bytes::new()));
        msr_mock
            .expect_fetch_raw_song()
            .returning(|_| Ok(Bytes::new()));

        let mock = Mock {
            song: Arc::new(song_mock),
            msr: Arc::new(msr_mock),
        };

        impl super::AddNewSongUseCase for Mock {}
        impl super::ProvideSongRepository for Mock {
            type SongRepository = MockUsesSongRepository;
            fn provide_song_repository(&self) -> &Self::SongRepository {
                &self.song
            }
        }
        impl super::ProvideMsrRepository for Mock {
            type MsrRepository = MockUsesMsrRepository;
            fn provide_msr_repository(&self) -> &Self::MsrRepository {
                &self.msr
            }
        }

        let result = super::fetch_and_create_song(&mock, 1.into()).await.unwrap();
        assert_eq!(result, super::song::Song::default());
    }
}
