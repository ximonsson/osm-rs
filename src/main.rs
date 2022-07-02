fn main() {
    let f = std::fs::File::open("../OSM.jl/data/oslo.osm").unwrap();
    let d = osm::Data::from_reader(f).unwrap();
    println!("{:?}", d);
}
