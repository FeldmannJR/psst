use std::sync::Arc;

use chrono::NaiveDate;
use druid::{im::Vector, Data, Lens};
use serde::Deserialize;

use crate::data::{ArtistLink, Cached, Image, Promise, Track};

#[derive(Clone, Data, Lens)]
pub struct AlbumDetail {
    pub album: Promise<Cached<Arc<Album>>, AlbumLink>,
}

#[derive(Clone, Data, Lens, Deserialize)]
pub struct Album {
    pub id: Arc<str>,
    pub name: Arc<str>,
    pub album_type: AlbumType,
    #[serde(default)]
    pub images: Vector<Image>,
    #[serde(default)]
    pub artists: Vector<ArtistLink>,
    #[serde(default)]
    pub copyrights: Vector<Copyright>,
    #[serde(default = "super::utils::default_str")]
    pub label: Arc<str>,
    #[serde(default)]
    #[serde(deserialize_with = "super::utils::deserialize_first_page")]
    pub tracks: Vector<Arc<Track>>,
    #[serde(deserialize_with = "super::utils::deserialize_date_option")]
    #[data(same_fn = "PartialEq::eq")]
    pub release_date: Option<NaiveDate>,
    #[data(same_fn = "PartialEq::eq")]
    pub release_date_precision: Option<DatePrecision>,
}

impl Album {
    pub fn release(&self) -> String {
        self.release_with_format(match self.release_date_precision {
            Some(DatePrecision::Year) | None => "%Y",
            Some(DatePrecision::Month) => "%B %Y",
            Some(DatePrecision::Day) => "%B %d, %Y",
        })
    }

    pub fn release_year(&self) -> String {
        self.release_with_format("%Y")
    }

    fn release_with_format(&self, format: &str) -> String {
        self.release_date
            .as_ref()
            .map(|date| date.format(format).to_string())
            .unwrap_or_else(|| '-'.to_string())
    }

    pub fn image(&self, width: f64, height: f64) -> Option<&Image> {
        Image::at_least_of_size(&self.images, width, height)
    }

    pub fn url(&self) -> String {
        format!("https://open.spotify.com/album/{id}", id = self.id)
    }

    pub fn link(&self) -> AlbumLink {
        AlbumLink {
            id: self.id.clone(),
            name: self.name.clone(),
            images: self.images.clone(),
        }
    }
}

#[derive(Clone, Debug, Data, Lens, Eq, PartialEq, Hash, Deserialize)]
pub struct AlbumLink {
    pub id: Arc<str>,
    pub name: Arc<str>,
    #[serde(default)]
    pub images: Vector<Image>,
}

impl AlbumLink {
    pub fn image(&self, width: f64, height: f64) -> Option<&Image> {
        Image::at_least_of_size(&self.images, width, height)
    }
}

#[derive(Clone, Debug, Data, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    Compilation,
    AppearsOn,
}

impl Default for AlbumType {
    fn default() -> Self {
        Self::Album
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Data, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

#[derive(Clone, Debug, Data, Lens, Deserialize)]
pub struct Copyright {
    pub text: Arc<str>,
    #[serde(rename = "type")]
    pub kind: CopyrightType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Data, Deserialize)]
pub enum CopyrightType {
    #[serde(rename = "C")]
    Copyright,
    #[serde(rename = "P")]
    Performance,
}
