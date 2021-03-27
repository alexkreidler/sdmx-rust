use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub type Sources = Vec<Source>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub id: String,
    pub data_content_type: Option<String>,
    pub url: String,
    pub name: String,
    pub supports: Option<Supports>,
    pub documentation: Option<String>,
    pub headers: Option<Headers>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Headers {
    #[serde(rename = "Accept")]
    pub accept: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Supports {
    pub preview: Option<bool>,
    pub agencyscheme: Option<bool>,
    pub categoryscheme: Option<bool>,
    pub codelist: Option<bool>,
    pub conceptscheme: Option<bool>,
    pub provisionagreement: Option<bool>,
    #[serde(rename = "structure-specific data")]
    pub structure_specific_data: Option<bool>,
    pub datastructure: Option<bool>,
}
