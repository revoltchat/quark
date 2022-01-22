use serde::{Deserialize, Serialize};

use crate::models::attachment::File;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ImageSize {
    Large,
    Preview,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub width: isize,
    pub height: isize,
    pub size: ImageSize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub url: String,
    pub width: isize,
    pub height: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TwitchType {
    Channel,
    Video,
    Clip,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BandcampType {
    Album,
    Track,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Special {
    None,
    YouTube {
        id: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        timestamp: Option<String>,
    },
    Twitch {
        content_type: TwitchType,
        id: String,
    },
    Spotify {
        content_type: String,
        id: String,
    },
    Soundcloud,
    Bandcamp {
        content_type: BandcampType,
        id: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special: Option<Special>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Video>,

    // #[serde(skip_serializing_if = "Option::is_none")]
    // opengraph_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    site_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colour: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Embed {
    Website(Metadata),
    Image(Image),
    Text(Text),
    None,
}
