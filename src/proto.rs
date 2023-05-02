pub mod items {
    include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
}

use flate2::{Decompress, FlushDecompress};
use items::blob::Data;
use items::*;
use prost::Message;
use std::io::{Read, Result};

#[macro_export]
macro_rules! coord {
    ($x: expr, $offset: expr, $granularity: expr) => {
        0.000000001 * ($offset + $granularity * $x) as f64
    };
}

#[derive(Debug)]
pub enum FileBlock {
    Header(HeaderBlock),
    Primitive(PrimitiveBlock),
}

const BYTES_BLOB_HEADER_SIZE: u64 = 4;
const MAX_BLOB_SIZE: usize = 2 ^ 25; // 32MB

/// Parse string table from `PrimitiveBlock`.
pub fn parse_str_tbl(pb: &PrimitiveBlock) -> Vec<String> {
    let mut st = Vec::<String>::with_capacity(pb.stringtable.s.len());
    for b in &pb.stringtable.s {
        let s = std::str::from_utf8(&b).unwrap();
        st.push(s.to_string());
    }
    st
}

/// Decode Blob data to get a FileBlock.
///
/// Depending on the compression the function will decode the underlying data of the `Blob` and return
/// a `FileBlock` item with the data..
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

pub struct FileBlockIterator {
    r: Box<dyn Read>,
    buf: Vec<u8>,
}

impl FileBlockIterator {
    /// Read some data from a source.
    ///
    /// This is just a convenience function because we read in several places.
    fn read(&mut self, n: u64) -> Result<usize> {
        self.buf.clear();
        self.r.by_ref().take(n).read_to_end(&mut self.buf)
    }

    /// Read the next `BlobHeader` from source.
    fn read_blob_header(&mut self) -> Result<BlobHeader> {
        // read blob header size
        let n = self.read(BYTES_BLOB_HEADER_SIZE).unwrap();
        if n == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "lol"));
        }
        let bhs: u32 = u32::from_be_bytes(
            self.buf[..BYTES_BLOB_HEADER_SIZE as usize]
                .try_into()
                .unwrap(),
        );

        // decode blob header
        self.read(bhs as u64).unwrap();
        let bh = BlobHeader::decode(self.buf.as_ref()).unwrap();
        Ok(bh)
    }

    /// Read next `Blob` from source.
    fn read_blob(&mut self, n: u64) -> Result<Blob> {
        self.read(n).unwrap();
        let blob = Blob::decode(self.buf.as_ref()).unwrap();
        Ok(blob)
    }

    pub fn from_reader(r: impl Read + 'static) -> FileBlockIterator {
        // create buffer
        let buf: Vec<u8> = Vec::with_capacity(MAX_BLOB_SIZE);
        FileBlockIterator {
            r: Box::new(r),
            buf: buf,
        }
    }
}

impl Iterator for FileBlockIterator {
    type Item = FileBlock;

    fn next(&mut self) -> Option<Self::Item> {
        // read blob header
        let header = match self.read_blob_header() {
            Ok(h) => h,

            // TODO
            // Should check better which type of error this is, just assuming
            // it is EOF here.
            Err(_) => return None,
        };

        // read blob
        let blob: Blob = self.read_blob(header.datasize as u64).unwrap();

        // decode the blob to file block
        Some(decode_blob(blob, header))
    }
}
