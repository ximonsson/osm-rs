use osm::proto::items;
use prost::Message;
use quick_xml;

fn foo(fp: &str) {
    let b = std::fs::read(fp).unwrap();
    let buf: &[u8] = &b;

    // get the size of the blobheader
    let n = u32::from_be_bytes(buf[..4].try_into().unwrap());
    println!("BlobHeader size: {}", n);

    // decode the BlobHeader
    let bh = items::BlobHeader::decode(&buf[4..(4 + n as usize)]).unwrap();
    println!("{:?}", bh);
}

fn read_xml(fp: &str) -> Result<(), quick_xml::de::DeError> {
    let _d = osm::xml::Data::from_reader(std::io::BufReader::new(std::fs::File::open(fp).unwrap()))
        .unwrap();
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    //read_xml(&args[1]).unwrap();

    foo(&args[1]);

    //println!("{:?}", _d);
    println!("pronto!");
}
