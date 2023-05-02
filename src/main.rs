use quick_xml;

fn foo(fp: &str) -> std::io::Result<()> {
    let f = std::fs::File::open(fp).unwrap();
    match osm::File::from_proto_reader(f) {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}

fn read_xml(fp: &str) -> Result<(), quick_xml::de::DeError> {
    let _d =
        osm::File::from_reader(std::io::BufReader::new(std::fs::File::open(fp).unwrap())).unwrap();
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let ext = std::path::Path::new(&args[1]).extension();
    match ext.unwrap().to_str() {
        Some("pbf") => foo(&args[1]).unwrap(),
        Some("osm") | Some("xml") => read_xml(&args[1]).unwrap(),
        Some(x) => println!("Unrecognized file type {}!", x),
        None => panic!("what?"),
    };

    println!("pronto!");
}
