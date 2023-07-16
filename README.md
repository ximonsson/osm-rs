# Openstreep Map Data In Rust

A crate for working with openstreetmap data.

Currently supports loading from both XML and protobuf.


## API

The main class is `osm::File` which contains the information loaded from a source file. This class has two constructors depending on the format of your data:

```rust
osm::File::from_reader(r: impl Read)  // for osm-xml data
```

```rust
osm::File::from_proto_reader(r: impl Read)  // for protobuf encoded data
```

You can check `src/main.rs` for example on how to parse data. When running `cargo build` a binary file is created that takes a local file path to an either OSM-XML or protobuf encoded data file. Depending on the file extension the binary will use the appropriate constructor.

## TODO

This crate is mostly a learning experience with rust so I do not take this too seriously.

### Multi-threaded Reading of Protobuf Files

The loading of protobuf encoded files is quite fast as it is, loading all data in Sweden takes about 15 seconds, but I would like to try and make it event faster using multi-threading.

[Here](https://github.com/ximonsson/osm2parquet/blob/2663da5dfeff2fca8b8c3181309c52bf4fe97050/src/main.rs#L5-L184) is an example of where i do this iterating over a `osm::proto::FileBlockIterator`. That code is however a bit long and messy maybe. Will just need to clean it up a little.

### Better Docs

Need to read up on how you are supposed to write good docs for rust crates.
