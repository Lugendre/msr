use url::Url;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MsrResponse<T> {
    pub code: i32,
    pub data: T,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SongSummaries {
    pub list: Vec<SongSummary>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SongSummary {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "albumCid")]
    pub belong_album_id: String,
    pub artists: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumSummary {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    pub cover_url: Url,
    #[serde(rename = "artistes")]
    pub artists: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "albumCid")]
    pub belong_album_id: String,
    pub source_url: Url,
    pub lyric_url: Option<Url>,
    pub artists: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    pub intro: String,
    pub belong: String,
    pub cover_url: Url,
    pub cover_de_url: Url,
    #[serde(rename = "artistes")]
    pub artists: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDetail {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    pub intro: String,
    pub belong: String,
    pub cover_url: Url,
    pub cover_de_url: Url,
    #[serde(rename = "songs")]
    pub song_list: Vec<SongShort>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SongShort {
    #[serde(rename = "cid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "artistes")]
    pub artists: Vec<String>,
}
