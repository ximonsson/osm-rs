use flate2::{Decompress, FlushDecompress};
use prost::Message;
use std::io::{Read, Result};

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
}

use items::blob::Data;
use items::*;

#[derive(Debug)]
enum FileBlock {
    HB(HeaderBlock),
    PB(PrimitiveBlock),
}

const BYTES_BLOB_HEADER_SIZE: u64 = 4;
const MAX_BLOB_SIZE: usize = 2 ^ 25; // 32MB

fn read(r: impl Read, n: u64, mut buf: &mut Vec<u8>) -> Result<usize> {
    buf.clear();
    r.take(n).read_to_end(&mut buf)
}

fn read_blob_header(mut r: impl Read, mut buf: &mut Vec<u8>) -> Result<BlobHeader> {
    // read blob header size
    read(r.by_ref(), BYTES_BLOB_HEADER_SIZE, &mut buf).unwrap();
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

fn parse_str_tbl(pb: &PrimitiveBlock) -> Vec<String> {
    let mut st = Vec::<String>::with_capacity(pb.stringtable.s.len());

    for b in &pb.stringtable.s {
        let s = std::str::from_utf8(&b).unwrap();
        st.push(s.to_string());
    }

    return st;
}

fn decode_blob(b: Blob, h: BlobHeader) {
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

    let msg = match h.r#type.as_str() {
        "OSMHeader" => FileBlock::HB(HeaderBlock::decode(buf.as_ref()).unwrap()),
        "OSMData" => FileBlock::PB(PrimitiveBlock::decode(buf.as_ref()).unwrap()),
        _ => panic!("unrecognized file block!"),
    };
    println!("{:?}", msg);

    let st: Option<Vec<String>> = match msg {
        FileBlock::PB(m) => Some(parse_str_tbl(&m)),
        FileBlock::HB(_) => None,
    };
    println!("{:?}", st);
}

pub fn from_reader(mut r: impl Read) -> Result<()> {
    // create buffer
    let mut buf: Vec<u8> = Vec::with_capacity(MAX_BLOB_SIZE);

    // read blob header
    let mut header = read_blob_header(r.by_ref(), &mut buf).unwrap();
    println!("{:?}", header);

    // read blob
    let mut blob = read_blob(r.by_ref(), header.datasize as u64, &mut buf).unwrap();
    //println!("{:?}", blob);

    // decode the blob to correct
    decode_blob(blob, header);

    println!("----------------------------");

    // read blob header
    header = read_blob_header(r.by_ref(), &mut buf).unwrap();
    println!("{:?}", header);

    // read blob
    blob = read_blob(r.by_ref(), header.datasize as u64, &mut buf).unwrap();
    //println!("{:?}", blob);

    // decode the blob to correct
    decode_blob(blob, header);

    println!("----------------------------");

    // read blob header
    header = read_blob_header(r.by_ref(), &mut buf).unwrap();
    println!("{:?}", header);

    // read blob
    blob = read_blob(r.by_ref(), header.datasize as u64, &mut buf).unwrap();
    //println!("{:?}", blob);

    // decode the blob to correct
    decode_blob(blob, header);

    Ok(())
}
