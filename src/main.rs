use osm::proto::items;
use prost::Message;
use quick_xml;

fn foo(fp: &str) {
    //let r = std::io::BufReader::new(std::fs::File::open(fp).unwrap());

    let b = std::fs::read(fp).unwrap();

    let x: &[u8] = &b;
    //let mut buf = std::io::Cursor::new(x);

    //let mut buf = bytes::Bytes::from(b);

    let bh = items::BlobHeader::decode(x).unwrap();
    println!("blob header: {:?}", bh);
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
