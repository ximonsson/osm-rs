use flate2::{Decompress, FlushDecompress};
use prost::Message;
use std::io::{Read, Result};

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
}

use items::blob::Data;
use items::*;

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

fn decode_blob(b: Blob, h: BlobHeader) {
    let n: usize = match b.raw_size {
        Some(x) => x as usize,
        None => 0,
    };

    let mut buf = vec![0; n];
    let status = match b.data {
        Some(Data::ZlibData(x)) => Decompress::new(true)
            .decompress(&x, &mut buf, FlushDecompress::Finish)
            .unwrap(),
        _ => todo!("support more"),
    };
    println!("{:?}", status);

    let msg = match h.r#type.as_str() {
        "OSMHeader" => Some(HeaderBlock::decode(buf.as_ref()).unwrap()),
        _ => todo!(),
        //"OSMData" => Some(PrimitiveBlock::decode(buf.as_ref()).unwrap()),
    };
    println!("{:?}", msg);
}

fn read_blob(mut r: impl Read, n: u64, mut buf: &mut Vec<u8>) -> Result<Blob> {
    read(r.by_ref(), n, &mut buf).unwrap();
    let blob = Blob::decode(buf.as_ref()).unwrap();
    Ok(blob)
}

pub fn from_reader(mut r: impl Read) -> Result<()> {
    // create buffer
    let mut buf: Vec<u8> = Vec::with_capacity(MAX_BLOB_SIZE);

    // read blob header
    let mut header = read_blob_header(r.by_ref(), &mut buf).unwrap();
    println!("{:?}", header);

    // read blob
    let mut blob = read_blob(r.by_ref(), header.datasize as u64, &mut buf).unwrap();
    println!("{:?}", blob);

    // decode the blob to correct
    decode_blob(blob, header);

    println!("----------------------------");

    // read blob header
    let mut header = read_blob_header(r.by_ref(), &mut buf).unwrap();
    println!("{:?}", header);

    // read blob
    let mut blob = read_blob(r.by_ref(), header.datasize as u64, &mut buf).unwrap();
    println!("{:?}", blob);

    Ok(())
}
