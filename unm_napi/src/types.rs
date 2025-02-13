use std::{borrow::Cow, collections::HashMap};

use napi_derive::napi;
pub use unm_types::SerializedIdentifier;

/// [napi-rs] The metadata of the artist of a song.
#[napi(object)]
pub struct Artist {
    /// The identifier of this artist.
    pub id: String,
    /// The name of this artist.
    pub name: String,
}

/// [napi-rs] The metadata of the album of a song.
#[napi(object)]
pub struct Album {
    /// The identifier of this artist.
    pub id: String,
    /// The name of this album.
    pub name: String,
}

/// [napi-rs] The metadata of a song.
#[napi(object)]
pub struct Song {
    /// The identifier of this song.
    pub id: String,
    /// The name of this song.
    pub name: String,
    /// The duration of this song.
    pub duration: Option<i64>,
    /// The artist of this song.
    pub artists: Vec<Artist>,
    /// The album of this song.
    pub album: Option<Album>,
    /// The context of this song.
    ///
    /// For example, the URI identifier of this song.
    pub context: Option<HashMap<String, String>>,
}

/// [napi-rs] The song identifier with the engine information.
#[napi(object)]
pub struct SongSearchInformation {
    /// The retrieve source of this song, for example: `bilibili`.
    pub source: String,
    /// The serialized identifier of this song.
    pub identifier: SerializedIdentifier,
    /// The details of this song.
    pub song: Option<Song>,
}

/// [napi-rs] The information of the song retrieved with `retrieve()`.
#[napi(object)]
pub struct RetrievedSongInfo {
    /// The retrieve source of this song, for example: `bilibili`.
    pub source: String,
    /// The URL of this song.
    pub url: String,
}

/// [napi-rs] The context.
#[napi(object)]
pub struct Context {
    /// The proxy URI
    pub proxy_uri: Option<String>,

    /// Whether to enable FLAC support.
    pub enable_flac: bool,

    /// The config for engines.
    pub config: Option<HashMap<String, String>>,
}

impl From<Artist> for unm_types::Artist {
    fn from(artist: Artist) -> Self {
        Self {
            id: artist.id,
            name: artist.name,
        }
    }
}

impl From<unm_types::Artist> for Artist {
    fn from(artist: unm_types::Artist) -> Self {
        Self {
            id: artist.id,
            name: artist.name,
        }
    }
}

impl From<Album> for unm_types::Album {
    fn from(album: Album) -> Self {
        Self {
            id: album.id,
            name: album.name,
        }
    }
}

impl From<unm_types::Album> for Album {
    fn from(album: unm_types::Album) -> Self {
        Self {
            id: album.id,
            name: album.name,
        }
    }
}

impl From<Song> for unm_types::Song {
    fn from(song: Song) -> Self {
        Self {
            id: song.id,
            name: song.name,
            duration: song.duration,
            artists: song.artists.into_iter().map(Into::into).collect(),
            album: song.album.map(Into::into),
            context: song.context,
        }
    }
}

impl From<unm_types::Song> for Song {
    fn from(song: unm_types::Song) -> Self {
        Self {
            id: song.id,
            name: song.name,
            duration: song.duration,
            artists: song.artists.into_iter().map(Into::into).collect(),
            album: song.album.map(Into::into),
            context: song.context,
        }
    }
}

impl From<unm_types::SongSearchInformation<'_>> for SongSearchInformation {
    fn from(song_information: unm_types::SongSearchInformation) -> Self {
        Self {
            source: song_information.source.to_string(),
            identifier: song_information.identifier,
            song: song_information.song.map(Into::into),
        }
    }
}

impl From<SongSearchInformation> for unm_types::SongSearchInformation<'_> {
    fn from(song_information: SongSearchInformation) -> Self {
        Self {
            source: Cow::Owned(song_information.source),
            identifier: song_information.identifier,
            song: song_information.song.map(Into::into),
        }
    }
}

impl From<unm_types::RetrievedSongInfo<'_>> for RetrievedSongInfo {
    fn from(song_information: unm_types::RetrievedSongInfo) -> Self {
        Self {
            source: song_information.source.to_string(),
            url: song_information.url,
        }
    }
}

impl Context {
    pub(crate) fn to_unm_context(&self) -> unm_types::Context {
        unm_types::Context {
            proxy_uri: self.proxy_uri.clone(),
            enable_flac: self.enable_flac,
            config: self.config.clone(),
        }
    }
}
