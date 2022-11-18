pub mod proto;
pub mod xml;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    id: i64,
    tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Way {
    id: i64,
    refs: Vec<i64>,
    tags: HashMap<String, String>,
}

impl Way {
    fn from_proto(w: &proto::items::Way, st: &Vec<String>) -> Self {
        //let tags: HashMap<u32, u32> = std::iter::zip(w.keys, w.vals).into();
        //let tags = HashMap::from_iter(std::iter::zip(w.keys, w.vals));
        let tags = HashMap::new();

        Way {
            id: w.id,
            refs: w.refs.clone(),
            tags: tags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Relation {
    id: i64,
}

#[derive(Debug, Clone)]
pub struct Data {
    nodes: HashMap<i64, Node>,
    ways: HashMap<i64, Way>,
    relations: HashMap<i64, Relation>,
}

impl Data {
    fn push_way(&mut self, w: Way) {
        self.ways.insert(w.id, w);
    }
}

#[cfg(test)]
mod tests {}
