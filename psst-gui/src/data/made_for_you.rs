use std::{iter::FromIterator, sync::Arc};

use crate::data::Playlist;
use druid::im::Vector;
use druid::{Data, Lens};

#[derive(Data, Clone, Hash, PartialEq, Eq, Debug)]
pub enum MadeForYouKind {
    GenreMix,
    DailyMix,
    ArtistMix,
    DecadeMix,
    UniquelyYours,
    Discover,
}
#[derive(Data, Lens, Clone)]
pub struct MadeForYouPlaylist {
    kind: MadeForYouKind,
    playlist: Playlist,
}

#[derive(Data, Lens, Clone)]
pub struct MadeForYou {
    playlists: Vector<MadeForYouPlaylist>,
}
impl MadeForYouPlaylist {
    pub fn new(kind: MadeForYouKind, playlist: Playlist) -> Self {
        MadeForYouPlaylist { kind, playlist }
    }
}

impl MadeForYouKind {
    pub fn from_id(name: Arc<str>) -> Option<Self> {
        match name.as_ref() {
            "my-mix-genres" => Some(MadeForYouKind::GenreMix),
            "made-for-x-dailymix" => Some(MadeForYouKind::DailyMix),
            "artist-seed-mixes" => Some(MadeForYouKind::ArtistMix),
            "my-mix-decades" => Some(MadeForYouKind::DecadeMix),
            "uniquely-yours-in-hub" => Some(MadeForYouKind::UniquelyYours),
            "made-for-x-discovery" => Some(MadeForYouKind::Discover),
            _ => Option::None,
        }
    }
    pub fn display_name(&self) -> String {
        match self{
            MadeForYouKind::GenreMix => "Genre Mixes",
            MadeForYouKind::DailyMix => "Daily Mixes",
            MadeForYouKind::ArtistMix => "Artist Mixes",
            MadeForYouKind::DecadeMix => "Decade Mixes",
            MadeForYouKind::UniquelyYours => "Uniquely Yours",
            MadeForYouKind::Discover => "Discover",
        }.to_string()
    }
}
impl FromIterator<MadeForYouPlaylist> for MadeForYou {
    fn from_iter<T: IntoIterator<Item = MadeForYouPlaylist>>(iter: T) -> Self {
        MadeForYou {
            playlists: Vector::from_iter(iter),
        }
    }
}

impl Lens<MadeForYou, Vector<Playlist>> for MadeForYouKind {
    fn with<V, F: FnOnce(&Vector<Playlist>) -> V>(&self, data: &MadeForYou, f: F) -> V {
        f(&data.get_by_kind(self))
    }

    fn with_mut<V, F: FnOnce(&mut Vector<Playlist>) -> V>(&self, data: &mut MadeForYou, f: F) -> V {
        f(&mut data.get_by_kind(self))
    }
}

impl MadeForYou {
    pub fn get_by_kind(&self, kind: &MadeForYouKind) -> Vector<Playlist> {
        self.playlists
            .iter()
            .filter(|el| el.kind.eq(kind))
            .map(|el| el.playlist.clone())
            .collect()
    }
}
