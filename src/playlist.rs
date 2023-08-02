use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub playlist_title: String,
    pub playlist_author: String,
    pub image: Option<String>,
    #[serde(rename = "syncURL")]
    pub sync_url: Option<String>,
    pub songs: Vec<Song>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub hash: String,
    pub song_name: String,
    pub difficulties: Vec<Difficulty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difficulty {
    pub characteristic: String,
    pub name: String,
}
