This requires a little bit of setup on your build host.

Requirements:
- protoc
- clang
- rust 2018
  - protobuf-codegen
    - cargo install protobuf-codegen
  - grpc-compiler
    - cargo install grpc-compiler
- Go v1.11 (Only required if we are building with "secure" feature enabled)

# Build

## w/ TLS support
The version of pingcap's grpc library that compiles w/ TLS support is only in their github repo at the time of writing.
```
git clone https://github.com/pingcap/grpc-rs.git
cd grpc-rs
git submodule update --init --recursive
```

## w/o TLS support
You need to switch the reference to the grpcio dependency. See Cargo.toml

## After building
## To run the examples, you'll need 2 terminals
- Terminal 1:
  - `cargo run --bin server` # Will return a port number for Terminal 2
  - ex.
  ```
  $ cargo run --bin server
      Finished dev [unoptimized + debuginfo] target(s) in 0.21s
       Running `target/debug/server`
  listening on 127.0.0.1:53571
  Press ENTER to exit...
  ```
- Terminal 2:
  - `cargo run --bin client -- <port number>`
  - ex.
  ```
  $ cargo run --bin client -- 53571
      Finished dev [unoptimized + debuginfo] target(s) in 0.11s
       Running `target/debug/client 53551`
  Ate items: SPAM items: EGGS and got charged $0.30
  ```
