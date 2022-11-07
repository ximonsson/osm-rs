use osm::proto::items;
use quick_xml;

fn foo(fp: &str) {}

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
