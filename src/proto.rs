pub mod items {
    include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
}

use crate::*;
use flate2::{Decompress, FlushDecompress};
use items::blob::Data;
use items::*;
use prost::Message;
use std::io::{Read, Result};

#[macro_export]
macro_rules! coord {
    ($x: expr, $offset: expr, $granularity: expr) => {
        0.000000001 * ($offset.unwrap_or(0) as i64 + $granularity.unwrap_or(100) as i64 * $x) as f64
    };
}

#[derive(Debug)]
enum FileBlock {
    Header(HeaderBlock),
    Primitive(PrimitiveBlock),
}

const BYTES_BLOB_HEADER_SIZE: u64 = 4;
const MAX_BLOB_SIZE: usize = 2 ^ 25; // 32MB

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
        if let Some(dense) = &g.dense {
            crate::Node::from_proto_dense_nodes(&dense, &st, &pb).for_each(|n| {
                es.push(Element::Node(n));
            })
        } else if g.ways.len() > 0 {
            g.ways
                .iter()
                .map(|w| crate::Way::from_proto(&w, &st))
                .for_each(|w| es.push(Element::Way(w)));
        } else if g.relations.len() > 0 {
            g.relations
                .iter()
                .map(|r| crate::Relation::from_proto(&r, &st))
                .for_each(|r| es.push(Element::Relation(r)));
        } else if g.nodes.len() > 0 {
            g.nodes
                .iter()
                .map(|n| crate::Node::from_proto(&n, &st, &pb))
                .for_each(|n| es.push(Element::Node(n)));
        } else if g.changesets.len() > 0 {
            // we ignore these
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

fn read(r: impl Read, n: u64, mut buf: &mut Vec<u8>) -> Result<usize> {
    buf.clear();
    r.take(n).read_to_end(&mut buf)
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

    if let FileBlock::Primitive(b) = block {
        decode_primitive_block(&b);
    }

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
    }

    Ok(())
}
