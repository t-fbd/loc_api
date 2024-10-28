use serde::{Serialize, Deserialize};

/// Represents the possible response formats for API requests.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Format {
    /// JSON format (`fo=json`).
    #[serde(rename = "json")]
    Json,
    /// YAML format (`fo=yaml`).
    #[serde(rename = "yaml")]
    Yaml,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

impl Format {
    /// Returns the corresponding slug used in the API URL for each format type.
    pub fn slug(&self) -> &'static str {
        match self {
            Format::Json => "json",
            Format::Yaml => "yaml",
        }
    }
}

/// Enum to represent specific format types for the `/{format}/` endpoint.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MediaType {
    /// Audio recordings (`/audio/`).
    Audio,
    /// Books or printed material (`/books/`).
    Books,
    /// Films and videos (`/film-and-videos/`).
    FilmAndVideos,
    /// Legislation (`/legislation/`).
    Legislation,
    /// Manuscripts or mixed material (`/manuscripts/`).
    Manuscripts,
    /// Maps (`/maps/`).
    Maps,
    /// Newspapers (`/newspapers/`).
    Newspapers,
    /// Photos, prints, or drawings (`/photos/`).
    Photos,
    /// Notated music such as sheet music (`/notated-music/`).
    NotatedMusic,
    /// Web archives (`/web-archives/`).
    WebArchives,
}

impl MediaType {
    /// Returns the corresponding slug used in the API URL for each format type.
    pub fn slug(&self) -> &'static str {
        match self {
            MediaType::Audio => "audio",
            MediaType::Books => "books",
            MediaType::FilmAndVideos => "film-and-videos",
            MediaType::Legislation => "legislation",
            MediaType::Manuscripts => "manuscripts",
            MediaType::Maps => "maps",
            MediaType::Newspapers => "newspapers",
            MediaType::Photos => "photos",
            MediaType::NotatedMusic => "notated-music",
            MediaType::WebArchives => "web-archives",
        }
    }
}
