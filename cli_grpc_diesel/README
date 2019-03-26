In this example, we want to demonstrate the interactions between Protobufs and Diesel.

The aim is to utilize protoc generated data structures, and carry that representation from a client over grpc to a backend and into the database through Diesel.


```
cargo install diesel_cli --features postgres
```

The example command line tool uses proto models on client and server-side.
The story:
The client side has an oil ordering tool.
(Oil, bc get it...? protobufs + diesel, har har...)

We'll have 2 initial actions:
* Order a user-specified quantity, and a user-specified product of oil to a refinery
  * For the sake of this narrative, oil for these jobs is going to be magical, and infinite. For the purposes of having a regular shaped object to add into the DB.
* Look at status of an order
  * In a high-level summary
    * List the shipments, and time of order
  * In a per-shipment detail
    * Show a report of the single order

We want to highlight the following interactions:
* Getting user input from the command line, and marshalling into a protobuf derived type (I'm calling this "proto-native").
* Connecting to a backend grpc server, and sending/recieving proto-native data 
* Receiving data from the client, and marshalling into a proto-native type
* Reading/Writing rust native types into the database

# Pre-Build




## w/ TLS support (Most likely the version you want)
The version of pingcap's grpc library that compiles w/ TLS support is only in their github repo at the time of writing.
```
git clone https://github.com/pingcap/grpc-rs.git
cd grpc-rs
git submodule update --init --recursive
```

## w/o TLS support
You need to switch the reference to the grpcio dependency. See Cargo.toml

## Compiling the protos, and importing proto rust code
The example `build.rs` is used to compile the protos.
In our Cargo.toml, there are references for the proto-rust code as a library, and a mod.rs for import paths to use in cli.rs and server.rs.

# The rough script

```
# Install protobuf and all its other stuff on Macos
brew install protobuf

# Init grpc-rs
git clone https://github.com/pingcap/grpc-rs.git
pushd grpc-rs
git submodule update --init --recursive
popd

# Start postgres
docker-compose up -d

pushd workspace

# Install the database schema
diesel migration run

# In 2 terminals:

# Terminal 1: Backend
cargo run --bin backend

# Terminal 2: Client
cargo run --bin client -- order 1 diesel
cargo run --bin client -- summary

```
