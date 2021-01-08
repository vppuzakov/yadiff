use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Categories {
    #[serde(alias = "dir")]
    DIRECTORY,

    #[serde(alias = "file")]
    FILE,
}

#[derive(Deserialize, Debug)]
pub struct Embedded {
    items: Vec<Resource>,
    limit: u16,
    offset: u16,
    total: u16,
}

#[derive(Deserialize, Debug)]
pub struct Resource {
    name: String,
    created: DateTime<FixedOffset>,
    modified: DateTime<FixedOffset>,

    #[serde(alias = "type")]
    category: Categories,

    #[serde(alias = "_embedded")]
    embedded: Option<Embedded>,
}
