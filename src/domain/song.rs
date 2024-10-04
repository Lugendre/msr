use bytes::Bytes;
use deriving_via::DerivingVia;
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::PathBuf;
use strum::EnumString;
use crate::errors::domain::DomainError;

// TODO: commonに分離
#[derive(DerivingVia)]
#[deriving(
    From,
    FromStr(via: u32),
    Into,
    Copy,
    Default,
    IntoInner(via: u32),
    Display(via: u32),
    Serialize(via: u32),
    Deserialize(via: u32),
    Eq(via: u32),
    Ord(via: u32),
    Hash(via: u32),
)]
pub struct Id<T>(#[underlying] u32, std::marker::PhantomData<T>);

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

pub type SongId = Id<Song>;

impl SongId {
    pub fn new (id: u32) -> Self {
        // TODO: バリデーション
        Self(id, std::marker::PhantomData)
    }
    pub fn try_new(s :String) -> Result<Self, DomainError> {
        // TODO: バリデーション
        s
            .parse::<Self>()
            .map_err(|_| DomainError::FailedToParseSongId { s: s.clone() })
    }
}

pub type AlbumId = Id<Album>;

impl AlbumId {
    pub fn new (id: u32) -> Self {
        // TODO: バリデーション
        Self(id, std::marker::PhantomData)
    }
    pub fn try_new(s: String) -> Result<Self, DomainError> {
        // TODO: バリデーション
        // TODO: エラーの詰め替え
        s.parse::<Self>()
            .map_err(|_| DomainError::FailedToParseAlbumId { s: s.clone() })
    }
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, EnumString)]
pub enum AudioFormat {
    #[default]
    Flac,
    Mp3,
    Wav,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AudioRawData {
    pub raw: Bytes, // reconstructの際に実際にデータを取る必要はないはずなので、ユースケース次第でOptionにするかも
    pub format: AudioFormat,
    pub save_path: PathBuf,
}

impl AudioRawData {
    pub fn try_new(raw: Bytes) -> Result<Self, DomainError> {
        // TODO: ファイルハッシュを計算してファイル名を生成する
        let save_path = PathBuf::from("");
        // TODO: ファイルフォーマットの判定
        let format = AudioFormat::Flac;
        Ok(Self {
            raw,
            format,
            save_path,
        })
    }

    pub fn reconstruct(raw: Bytes, format: AudioFormat, save_path: PathBuf) -> Self {
        Self {
            raw,
            format,
            save_path,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Song {
    pub id: SongId,
    pub name: String,
    pub belong_album_id: AlbumId,
    pub track_number: u8,
    pub disk_number: u8,
    pub source: AudioRawData,
    pub artists: Vec<String>,
}

impl Song {
    pub fn try_new(
        id: SongId,
        name: String,
        belong_album_id: AlbumId,
        song_list: &[SongId], // Non-empty
        source: AudioRawData,
        artists: Vec<String>,
    ) -> Result<Self, DomainError> {
        // TODO: 各引数のバリデーション
        // TODO: song_listが空でないかチェックする
        let ix = song_list
            .iter()
            .position(|&song_id| song_id == id)
            .ok_or(DomainError::FailedToGetSongInAlbum {song_id: id, album_id: belong_album_id})?;
        let track_number = (ix + 1) as u8; // FIXME: エラーハンドリング
        let disk_number = 1; // 1枚組のアルバム以外ので
        Ok(Self {
            id,
            name,
            belong_album_id,
            track_number,
            disk_number,
            source,
            artists,
        })
    }

    pub fn try_reconstruct(
        id: SongId,
        name: String,
        belong_album_id: AlbumId,
        track_number: u8,
        disk_number: u8,
        source: AudioRawData,
        artists: Vec<String>,
    ) -> Result<Self, DomainError> {
        // TODO: 各引数のバリデーション
        Ok(Self {
            id,
            name,
            belong_album_id,
            track_number,
            disk_number,
            source,
            artists,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Album {
    pub id: AlbumId,
    pub name: String,
    pub total_tracks: u8,
    pub total_disks: u8,
    pub intro: String, // サイズ的にはshort stringでいいからもっと効率的なデータ構造がほしい
    pub belong: OriginGame,
    pub cover_image: Bytes,
    pub artists: Vec<String>,
    pub song_list: Vec<SongId>,
}

impl Album {
    pub fn try_new(
        id: AlbumId,
        name: String,
        intro: String,
        belong: OriginGame,
        cover_image: Bytes,
        artists: Vec<String>,
        song_list: Vec<SongId>,
    ) -> Result<Self, DomainError> {
        // TODO: 各引数のバリデーション
        Ok(Self {
            id,
            name,
            total_tracks: song_list.len() as u8,
            total_disks: 1, // 1枚組のアルバム以外ないので
            intro,
            belong,
            cover_image,
            artists,
            song_list,
        })
    }

    pub fn try_reconstruct(
        id: AlbumId,
        name: String,
        total_tracks: u8,
        total_disks: u8,
        intro: String,
        belong: OriginGame,
        cover_image: Bytes,
        artists: Vec<String>,
        song_list: Vec<SongId>,
    ) -> Result<Self, DomainError> {
        // TODO: 各引数のバリデーション
        Ok(Self {
            id,
            name,
            total_tracks,
            total_disks,
            intro,
            belong,
            cover_image,
            artists,
            song_list,
        })
    }
}

#[derive(Debug, Clone, Hash, Default, PartialEq, Eq, PartialOrd, Ord, strum::EnumString)]
pub enum OriginGame {
    #[default]
    #[strum(serialize = "arknights")]
    Arknights,
}

impl OriginGame {
    pub fn try_new(s: String) -> Result<Self, DomainError> {
        s.parse::<Self>()
            .map_err(|_| DomainError::FailedToParseOriginGame { s: s.clone() })
    }
}
