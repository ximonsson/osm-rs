use quick_xml::de::{from_reader, DeError};
use serde::{Deserialize, Serialize};
pub mod proto;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    k: String,
    v: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: i64,
    lat: f64,
    lon: f64,
    #[serde(rename = "tag", default)]
    tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRef {
    r#ref: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Way {
    id: i64,
    #[serde(rename = "nd")]
    nodes: Vec<NodeRef>,
    #[serde(rename = "tag", default)]
    tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    r#ref: i64,
    role: String,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    id: i64,
    #[serde(rename = "tag", default)]
    tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "node")]
    nodes: Vec<Node>,
    #[serde(rename = "way")]
    ways: Vec<Way>,
    #[serde(rename = "relation")]
    relations: Vec<Relation>,
}

impl File {
    // Parse OSM-XML file.
    pub fn from_reader(r: impl std::io::BufRead) -> Result<Self, DeError> {
        from_reader(r)
    }
}

#[cfg(test)]
mod tests {}
