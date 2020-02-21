extern crate futures;
extern crate grpcio;
extern crate protobuf;


pub mod refinery;
pub mod refinery_grpc;

// This is a hack to support google's empty type
// https://github.com/pingcap/grpc-rs/issues/276#issuecomment-468731608
// Rust's other grpc competition might support this better...
pub mod empty {
    pub use protobuf::well_known_types::Empty;
}
