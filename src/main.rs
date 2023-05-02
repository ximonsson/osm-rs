use quick_xml;

fn foo(fp: &str) -> std::io::Result<osm::File> {
    let f = std::fs::File::open(fp).unwrap();
    osm::File::from_proto_reader(f)
}

fn read_xml(fp: &str) -> Result<osm::File, quick_xml::de::DeError> {
    osm::File::from_reader(std::io::BufReader::new(std::fs::File::open(fp).unwrap()))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let ext = std::path::Path::new(&args[1]).extension();
    let data: osm::File = match ext.unwrap().to_str() {
        Some("pbf") => foo(&args[1]).unwrap(),
        Some("osm") | Some("xml") => read_xml(&args[1]).unwrap(),
        Some(x) => panic!("Unrecognized file type {}!", x),
        None => panic!("what?"),
    };

    println!("{:?}", data);

    println!("pronto!");
}
