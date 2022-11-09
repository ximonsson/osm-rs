use prost::Message;
use std::io::Read;

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
}

const BYTES_BLOB_HEADER_SIZE: u64 = 4;
const MAX_BLOB_SIZE: usize = 2 ^ 25; // 32MB

pub fn from_reader(mut r: impl std::io::Read + std::io::Seek) -> std::io::Result<()> {
    // create buffer
    let mut buf: Vec<u8> = Vec::with_capacity(MAX_BLOB_SIZE);

    // read blob header size
    r.by_ref()
        .take(BYTES_BLOB_HEADER_SIZE)
        .read_to_end(&mut buf)?;
    let bhs: u32 = u32::from_be_bytes(buf[..BYTES_BLOB_HEADER_SIZE as usize].try_into().unwrap());

    // decode blob header
    buf.clear();
    r.by_ref().take(bhs as u64).read_to_end(&mut buf)?;
    let bh = items::BlobHeader::decode(buf.as_ref()).unwrap();
    println!("{:?}", bh);

    // decode blob
    buf.clear();
    r.by_ref()
        .take(bh.datasize as u64)
        .read_to_end(&mut buf)
        .unwrap();
    let blob = items::Blob::decode(buf.as_ref()).unwrap();
    println!("{:?}", blob);

    Ok(())
}
