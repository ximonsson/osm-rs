use quick_xml::de::{from_reader, DeError};
use serde::{Deserialize, Serialize};
pub mod proto;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    k: String,
    v: String,
}

impl Tag {
    pub fn from_dense_nodes_kvs(kvs: &Vec<i32>, st: &Vec<String>, j: &mut usize) -> Vec<Self> {
        let mut i: usize = *j;
        let mut k: &str;
        let mut v: &str;
        let mut tags: Vec<Self> = Vec::with_capacity(kvs.len());

        while i < kvs.len() {
            if kvs[i] == 0 {
                i += 1;
                break;
            }
            k = &st[kvs[i] as usize];
            i += 1;
            v = &st[kvs[i] as usize];
            i += 1;
            tags.push(Tag {
                k: k.into(),
                v: v.into(),
            });
        }

        *j = i;
        tags
    }

    pub fn from_kvs(k: &Vec<u32>, v: &Vec<u32>, st: &Vec<String>) -> Vec<Self> {
        std::iter::zip(k, v)
            .map(|(k, v)| Tag {
                k: (&st[*k as usize]).into(),
                v: (&st[*v as usize]).into(),
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: i64,
    lat: f64,
    lon: f64,
    #[serde(rename = "tag", default)]
    tags: Vec<Tag>,
}

impl Node {
    pub fn from_proto_dense_nodes() -> Vec<Self> {
        vec![]
    }
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

impl Way {
    pub fn from_proto(w: &proto::items::Way, st: &Vec<String>) -> Self {
        let mut i: i64 = 0;
        let ns = w
            .refs
            .iter()
            .map(|r| {
                i += r;
                NodeRef { r#ref: i }
            })
            .collect();

        Self {
            id: w.id,
            nodes: ns,
            tags: Tag::from_kvs(&w.keys, &w.vals, st),
        }
    }
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
