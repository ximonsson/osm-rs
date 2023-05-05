use quick_xml::de::from_reader;
use serde::{Deserialize, Serialize};
pub mod proto;

#[derive(Debug)]
pub enum Error {
    XMLParseError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub k: String,
    pub v: String,
}

impl Tag {
    /// Create vector of tags from protobuf encoded source specifically for dense encoded
    /// nodes.
    fn from_dense_nodes_kvs(kvs: &Vec<i32>, st: &Vec<String>, j: &mut usize) -> Vec<Self> {
        let mut i: usize = *j;
        let mut k: &str;
        let mut v: &str;

        let mut tags: Vec<Self> = vec![];

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

    /// Create vector of tags from key value pair coming from protobuf encoded source.
    fn from_kvs(k: &Vec<u32>, v: &Vec<u32>, st: &Vec<String>) -> Vec<Self> {
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
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
}

impl Node {
    /// Create Iterator of Nodes from encoded dense nodes from protobuf source.
    fn from_proto_dense_nodes<'a>(
        dense: &'a proto::items::DenseNodes,
        st: &'a Vec<String>,
        pb: &'a proto::items::PrimitiveBlock,
    ) -> impl std::iter::Iterator<Item = Self> + 'a {
        // increment values
        let mut id: i64 = 0;
        let mut lat: i64 = 0;
        let mut lon: i64 = 0;

        // TODO
        // should these values not unwrap to their default values already?
        let offlon = pb.lon_offset.unwrap_or(0) as i64;
        let offlat = pb.lat_offset.unwrap_or(0) as i64;
        let granularity = pb.granularity.unwrap_or(100) as i64;

        // tag index
        let mut ti: usize = 0;

        std::iter::zip(&dense.id, std::iter::zip(&dense.lat, &dense.lon)).map(
            move |(i, (phi, lam))| {
                id += i;
                lat += phi;
                lon += lam;

                Node {
                    id: id,
                    lat: coord!(lat, offlat, granularity),
                    lon: coord!(lon, offlon, granularity),
                    tags: Tag::from_dense_nodes_kvs(&dense.keys_vals, &st, &mut ti),
                }
            },
        )
    }

    /// Create new Node from protobuf encoded node.
    fn from_proto(
        n: &proto::items::Node,
        st: &Vec<String>,
        pb: &proto::items::PrimitiveBlock,
    ) -> Self {
        // TODO
        // should these values not unwrap to their default values already?
        let offlon = pb.lon_offset.unwrap_or(0) as i64;
        let offlat = pb.lat_offset.unwrap_or(0) as i64;
        let granularity = pb.granularity.unwrap_or(100) as i64;

        Node {
            id: n.id,
            tags: Tag::from_kvs(&n.keys, &n.vals, st),
            lat: coord!(n.lat, offlat, granularity),
            lon: coord!(n.lon, offlon, granularity),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRef {
    pub r#ref: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Way {
    pub id: i64,
    #[serde(rename = "nd")]
    pub nodes: Vec<NodeRef>,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
}

impl Way {
    /// Return Way element from protobuf encoded source.
    fn from_proto(w: &proto::items::Way, st: &Vec<String>) -> Self {
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
    pub r#ref: i64,
    pub role: String,
    pub r#type: String,
}

impl Member {
    /// Return vector of members belonging to a protobuf encoded Relation.
    fn from_proto(
        ids: &Vec<i64>,
        roles: &Vec<i32>,
        types: &Vec<i32>,
        st: &Vec<String>,
    ) -> Vec<Self> {
        let mut i: i64 = 0;
        std::iter::zip(std::iter::zip(ids, roles), types)
            .map(|((id, rl), t)| {
                i += id;
                Member {
                    r#ref: i,
                    role: (&st[*rl as usize]).into(),
                    r#type: proto::items::relation::MemberType::from_i32(*t)
                        .unwrap()
                        .as_str_name()
                        .into(),
                }
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub id: i64,
    #[serde(rename = "tag", default)]
    pub tags: Vec<Tag>,
    #[serde(rename = "member", default)]
    pub members: Vec<Member>,
}

impl Relation {
    /// New Relation from protobuf encoded source.
    fn from_proto(r: &proto::items::Relation, st: &Vec<String>) -> Self {
        Relation {
            id: r.id,
            tags: Tag::from_kvs(&r.keys, &r.vals, st),
            members: Member::from_proto(&r.memids, &r.roles_sid, &r.types, st),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "node")]
    pub nodes: Vec<Node>,
    #[serde(rename = "way")]
    pub ways: Vec<Way>,
    #[serde(rename = "relation")]
    pub relations: Vec<Relation>,
}

impl File {
    /// Parse OSM-XML source.
    pub fn from_reader(r: impl std::io::Read) -> Result<Self> {
        match from_reader(std::io::BufReader::new(r)) {
            Ok(f) => Ok(f),
            Err(e) => Err(Error::XMLParseError(format!("XML parse error: {}", e))),
        }
    }

    /// Parse protobuf source.
    pub fn from_proto_reader(r: impl std::io::Read + 'static) -> Result<Self> {
        // allocate for data
        let mut nodes: Vec<Node> = vec![];
        let mut ways: Vec<Way> = vec![];
        let mut relations: Vec<Relation> = vec![];

        // iterate over all file blocks in the reader
        proto::FileBlockIterator::from_reader(r).for_each(|block| {
            // if file block it means we got data
            if let proto::FileBlock::Primitive(b) = block {
                let st = proto::parse_str_tbl(&b);

                b.primitivegroup.iter().for_each(|g| {
                    if let Some(dense) = &g.dense {
                        Node::from_proto_dense_nodes(&dense, &st, &b).for_each(|n| nodes.push(n));
                    } else if g.ways.len() > 0 {
                        g.ways
                            .iter()
                            .for_each(|w| ways.push(Way::from_proto(&w, &st)));
                    } else if g.relations.len() > 0 {
                        g.relations
                            .iter()
                            .for_each(|r| relations.push(Relation::from_proto(&r, &st)));
                    } else if g.nodes.len() > 0 {
                        g.nodes
                            .iter()
                            .for_each(|n| nodes.push(Node::from_proto(&n, &st, &b)));
                    } else if g.changesets.len() > 0 {
                        // we ignore these
                    }
                });
            }
        });

        Ok(File {
            nodes: nodes,
            ways: ways,
            relations: relations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proto() -> Result<()> {
        File::from_proto_reader(std::fs::File::open("./data/andorra-latest.osm.pbf").unwrap())?;
        Ok(())
    }

    #[test]
    fn test_osm_xml() -> Result<()> {
        File::from_reader(std::fs::File::open("./data/friesenstrasse.osm").unwrap())?;
        Ok(())
    }
}
