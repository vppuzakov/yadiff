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
    pub items: Vec<Resource>,
    pub limit: u16,
    pub offset: u32,
    pub total: u32,
}

#[derive(Deserialize, Debug)]
pub struct Resource {
    pub name: String,
    pub created: DateTime<FixedOffset>,
    pub modified: DateTime<FixedOffset>,

    #[serde(alias = "type")]
    pub category: Categories,

    #[serde(alias = "_embedded")]
    pub embedded: Option<Embedded>,
}
