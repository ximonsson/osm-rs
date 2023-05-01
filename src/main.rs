use osm::proto;
use quick_xml;

fn foo(fp: &str) -> std::io::Result<()> {
    let f = std::fs::File::open(fp).unwrap();
    match proto::from_reader(f) {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}

fn read_xml(fp: &str) -> Result<(), quick_xml::de::DeError> {
    let _d =
        osm::File::from_reader(std::io::BufReader::new(std::fs::File::open(fp).unwrap())).unwrap();
    //println!("{:?}", _d);
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    //read_xml(&args[1]).unwrap();
    foo(&args[1]).unwrap();

    println!("pronto!");
}
