extern crate protoc_grpcio;

fn main() {
    // The path looks like it references one directory up
    // because the protos dir is at the same level as this file.
    //
    // This build.rs file is shared by client + backend crates,
    // so I don't have to maintain 2 copies of this file
    //
    // However, the root is defined from the perspective of those crates.
    let proto_root = "../protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(&["shipment.proto"], &[proto_root], &proto_root, None)
        .expect("Failed to compile gRPC definitions!");
}
