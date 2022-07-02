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
    pub fn from_reader(r: impl std::io::Read) -> Result<Self, serde_xml_rs::Error> {
        serde_xml_rs::from_reader(r)
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
