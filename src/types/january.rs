use linkify::{LinkFinder, LinkKind};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::{models::attachment::File, Error, Result};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum ImageSize {
    Large,
    Preview,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub width: isize,
    pub height: isize,
    pub size: ImageSize,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct Video {
    pub url: String,
    pub width: isize,
    pub height: isize,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum TwitchType {
    Channel,
    Video,
    Clip,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub enum BandcampType {
    Album,
    Track,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
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

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "type")]
pub enum Embed {
    Website(Metadata),
    Image(Image),
    Text(Text),
    None,
}

impl Embed {
    pub async fn generate(content: String, host: &str, max_embeds: usize) -> Result<Vec<Embed>> {
        lazy_static! {
            static ref RE_CODE: Regex = Regex::new("```(?:.|\n)+?```|`(?:.|\n)+?`").unwrap();
            static ref RE_IGNORED: Regex = Regex::new("(<http.+>)").unwrap();
        }

        // Ignore code blocks.
        let content = RE_CODE.replace_all(&content, "");

        // Ignore all content between angle brackets starting with http.
        let content = RE_IGNORED.replace_all(&content, "");

        let content = content
            // Ignore quoted lines.
            .split('\n')
            .map(|v| {
                if let Some(c) = v.chars().next() {
                    if c == '>' {
                        return "";
                    }
                }

                v
            })
            .collect::<Vec<&str>>()
            .join("\n");

        let mut finder = LinkFinder::new();
        finder.kinds(&[LinkKind::Url]);

        let links: HashSet<String> = finder
            .links(&content)
            .take(max_embeds)
            .map(|x| x.as_str().to_string())
            .collect();

        if links.is_empty() {
            return Err(Error::LabelMe);
        }

        // ! FIXME: batch request to january?
        let mut embeds: Vec<Embed> = Vec::new();
        let client = reqwest::Client::new();
        for link in links {
            let result = client
                .get(&format!("{}/embed", host))
                .query(&[("url", link)])
                .send()
                .await;

            if result.is_err() {
                continue;
            }

            let response = result.unwrap();
            if response.status().is_success() {
                let res: Embed = response.json().await.map_err(|_| Error::InvalidOperation)?;
                embeds.push(res);
            }
        }

        // Prevent database update when no embeds are found.
        if !embeds.is_empty() {
            Ok(embeds)
        } else {
            Err(Error::LabelMe)
        }
    }
}
