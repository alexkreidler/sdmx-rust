use serde::{Deserialize, Serialize};
use std::{fmt::Debug, time::Duration};

pub type Sources = Vec<Source>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_content_type: Option<String>,
    pub url: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports: Option<Supports>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,

    /// Accept headers for structure queries (e.g. dataflows, datastructure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structural_accept: Option<Accept>,

    #[serde(default)]
    pub response_content_types: Vec<String>,

    /// Accept headers for data queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_accept: Option<Accept>,

    #[serde(default)]
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed: Vec<Duration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Accept {
    /// Accept headers with 200 status
    pub supported_accept_headers: Vec<String>,

    // Accept headers with errors
    pub denied_accept_headers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Headers {
    #[serde(rename = "Accept")]
    pub accept: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Supports {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agencyscheme: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categoryscheme: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codelist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conceptscheme: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisionagreement: Option<bool>,
    #[serde(rename = "structure-specific data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_specific_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastructure: Option<bool>,
}
