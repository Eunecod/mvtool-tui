// source/models/mod.rs

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
pub struct Root {
    #[serde(default)]
    pub projects: Vec<Project>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub selected: bool,
    pub destination_path: String,

    #[serde(default)]
    pub configures: Vec<Configure>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Configure {
    pub name: String,
    pub source_path: String,
    pub selected: bool,
    pub clean_destination: bool,

    #[serde(default)]
    pub extension_mask: Vec<String>,

    #[serde(default)]
    pub components: Vec<Component>,

    #[serde(default)]
    pub scripts: Vec<Script>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Component {
    pub name: String,
    pub selected: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Script {
    pub name: String,
    pub command: String,
}