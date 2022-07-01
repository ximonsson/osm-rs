use serde::{Deserialize, Serialize};
use serde_xml_rs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: u64,
    lat: f32,
    lon: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Way {
    id: u64,
    #[serde(rename = "$value")]
    nodes: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub nodes: Vec<Node>,
    pub ways: Vec<Way>,
    pub relations: Vec<Relation>,
}

pub struct Error {
    msg: String,
}

impl Data {
    // Creat an empty `Data` object.
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            ways: vec![],
            relations: vec![],
        }
    }

    // Parse OSM-XML file.
    pub fn from_filepath(fp: &str) -> Result<Self, E>
    where
        E: Error,
    {
        serde_xml_rs::from_reader(std::fs::File::open(fp)?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
