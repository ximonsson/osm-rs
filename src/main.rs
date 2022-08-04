fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }

    let _d = osm::Data::from_reader(std::io::BufReader::new(
        std::fs::File::open(&args[1]).unwrap(),
    ))
    .unwrap();

    println!("{:?}", _d);
    println!("pronto!");
}
