use prost::Message;
use std::io::Read;

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
}

const BYTES_BLOB_HEADER_SIZE: u64 = 4;
const MAX_BLOB_SIZE: usize = 32 * 2 ^ 20; // 32MB

pub fn from_reader(mut r: impl std::io::Read) -> std::io::Result<()> {
    // create buffer
    let mut buf = [0; MAX_BLOB_SIZE];

    // read blob header size
    r.by_ref().take(BYTES_BLOB_HEADER_SIZE).read(&mut buf)?;
    let bhs: u32 = u32::from_be_bytes(buf[..BYTES_BLOB_HEADER_SIZE as usize].try_into().unwrap());

    // decode blob header
    r.by_ref().take(bhs as u64).read(&mut buf)?;
    let bh = items::BlobHeader::decode(&buf[..bhs as usize]).unwrap();

    println!("{:?}", bh);

    Ok(())
}
