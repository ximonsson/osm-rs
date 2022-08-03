fn main() {
    let f = std::fs::File::open("../data/osm/jardim.osm").unwrap();
    let d = osm::Data::from_reader(f).unwrap();
    println!("{:?}", d);
}
