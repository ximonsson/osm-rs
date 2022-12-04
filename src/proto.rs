use flate2::{Decompress, FlushDecompress};
use prost::Message;
use std::io::{Read, Result};

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
}

use items::blob::Data;
use items::*;

use crate::*;

#[derive(Debug)]
enum FileBlock {
    Header(HeaderBlock),
    Primitive(PrimitiveBlock),
}

const BYTES_BLOB_HEADER_SIZE: u64 = 4;
const MAX_BLOB_SIZE: usize = 2 ^ 25; // 32MB

fn read(r: impl Read, n: u64, mut buf: &mut Vec<u8>) -> Result<usize> {
    buf.clear();
    r.take(n).read_to_end(&mut buf)
}

fn parse_str_tbl(pb: &PrimitiveBlock) -> Vec<String> {
    let mut st = Vec::<String>::with_capacity(pb.stringtable.s.len());

    for b in &pb.stringtable.s {
        let s = std::str::from_utf8(&b).unwrap();
        st.push(s.to_string());
    }

    st
}

fn decode_primitive_block(pb: &PrimitiveBlock) -> Vec<Element> {
    let st = parse_str_tbl(&pb);
    let mut es: Vec<Element> = Vec::with_capacity(pb.primitivegroup.len() * 8000);

    for g in &pb.primitivegroup {
        if let Some(dnodes) = &g.dense {
            let n = dnodes.id.len();

            let mut id: i64 = 0;
            let mut lat: i64 = 0;
            let mut lon: i64 = 0;

            // tag index
            let mut ti: usize = 0;

            for i in 0..n {
                id += dnodes.id[i];
                lat += dnodes.lat[i];
                lon += dnodes.lon[i];

                let tags = Tag::from_dense_nodes_kvs(&dnodes.keys_vals, &st, &mut ti);

                es.push(Element::Node(crate::Node {
                    id: id,
                    lat: 0.000000001
                        * (pb.lat_offset.unwrap_or(100) as i64
                            + (pb.granularity.unwrap_or(0) as i64 * lat))
                            as f64,
                    lon: 0.000000001
                        * (pb.lon_offset.unwrap_or(100) as i64
                            + (pb.granularity.unwrap_or(0) as i64 * lon))
                            as f64,
                    tags: tags,
                }));
            }
        } else if g.ways.len() > 0 {
            g.ways
                .iter()
                .map(|w| crate::Way::from_proto(&w, &st))
                .map(|w| es.push(Element::Way(w)));
        } else if g.relations.len() > 0 {
            //println!("we got some relations! {} in total", g.relations.len());
        } else if g.nodes.len() > 0 {
            //println!("we got some nodes! {} in total", g.nodes.len());
        } else if g.changesets.len() > 0 {
            //println!("we got some changesets! {} in total", g.changesets.len());
        }
    }

    es
}

fn decode_blob(b: Blob, h: BlobHeader) -> FileBlock {
    let n: usize = match b.raw_size {
        Some(x) => x as usize,
        None => 0,
    };

    let buf = match b.data {
        Some(Data::ZlibData(x)) => {
            let mut buf = vec![0; n];
            Decompress::new(true)
                .decompress(&x, &mut buf, FlushDecompress::Finish)
                .unwrap();
            buf
        }
        Some(Data::Raw(x)) => x,
        _ => todo!("support more"),
    };

    let msg: FileBlock = match h.r#type.as_str() {
        "OSMHeader" => FileBlock::Header(HeaderBlock::decode(buf.as_ref()).unwrap()),
        "OSMData" => FileBlock::Primitive(PrimitiveBlock::decode(buf.as_ref()).unwrap()),
        x => panic!("[{}] unrecognized file block!", x),
    };

    msg
}

fn read_blob_header(mut r: impl Read, mut buf: &mut Vec<u8>) -> Result<BlobHeader> {
    // read blob header size
    let n = read(r.by_ref(), BYTES_BLOB_HEADER_SIZE, &mut buf).unwrap();
    if n == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "lol"));
    }
    let bhs: u32 = u32::from_be_bytes(buf[..BYTES_BLOB_HEADER_SIZE as usize].try_into().unwrap());

    // decode blob header
    read(r.by_ref(), bhs as u64, &mut buf).unwrap();
    let bh = BlobHeader::decode(buf.as_ref()).unwrap();
    Ok(bh)
}

fn read_blob(mut r: impl Read, n: u64, mut buf: &mut Vec<u8>) -> Result<Blob> {
    read(r.by_ref(), n, &mut buf).unwrap();
    let blob = Blob::decode(buf.as_ref()).unwrap();
    Ok(blob)
}

fn step_reader(mut r: impl Read, mut buf: &mut Vec<u8>) -> Result<()> {
    // read blob header
    let header = match read_blob_header(r.by_ref(), &mut buf) {
        Ok(h) => h,
        Err(e) => return Err(e),
    };
    println!("{:?}", header);

    // read blob
    let blob = read_blob(r.by_ref(), header.datasize as u64, &mut buf).unwrap();

    // decode the blob to correct
    let block = decode_blob(blob, header);

    match block {
        FileBlock::Header(_) => println!("HeaderBlock"),
        FileBlock::Primitive(b) => {
            println!("PrimitiveBlock");
            decode_primitive_block(&b);
        }
    };

    Ok(())
}

pub fn from_reader(mut r: impl Read) -> Result<()> {
    // create buffer
    let mut buf: Vec<u8> = Vec::with_capacity(MAX_BLOB_SIZE);

    loop {
        if let Err(_) = step_reader(r.by_ref(), &mut buf) {
            println!("done!");
            break;
        }
        println!("--------------------------------------");
    }

    Ok(())
}
