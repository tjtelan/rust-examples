# Rust examples

This repo consists of the example Rust binaries I write or copy/pasted. The intentions are to get familiar with using opinionated features of new crates.

This repo is organized into Rust workspaces, but you may need to `cargo run` within a specific crate after following instructions within the crate's README.

**Disclaimer**: The crates I use in these examples do not necessarily represent a recommendation.

## Description of example crates
### cli-clap
This directly uses the [clap.rs](https://crates.io/crates/clap) crate using an external yaml to define the cli schema.

### cli-clap-grpc-pingcap-db-diesel 
This is the crate used in my blog post: [Using a database + gRPC with Rust](https://tjtelan.com/blog/using-a-database-grpc-with-rust/)

This crate is excluded from the workspace build. See the [README.md](cli-clap-grpc-pingcap-db-diesel/README.md) and follow the manual steps to build. 

It uses [clap.rs](https://crates.io/crates/clap) with its yaml schema feature, Pingcap's grpc library, and Diesel for postgresql support.

### consul
** Not currently compiling via workspace**

### db-postgres-diesel
This crate use a [docker-compose.yml](db-diesel-postgres/docker-compose.yml) to start a postgresqldb server. You may need `diesel-cli` installed so you can instantiate the pgsql server with the example database migration.

### docker-shiplift
Uses `shiplift` and `yaml-rust` crates. Reads a yaml file with some configurable info for pulling images, creating, starting and execing commands into a container.

### grpc-pingcap
This crate has a few external requirements (such as installing Golang) and manual steps for building the examples. See the crate's [README.md](grpc-pingcap/README.md)

### nsq
This crate uses [docker-compose.yml](nsq/docker-compose.yml) to start an `nsq` container. There are two binaries, `nsq-producer` and `nsq-consumer` which uses `tokio-core` and `future` v0.1

### proc-macro
This crate has an implementation of an [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros) using `syn` and `quote` and `proc-macro2`.

### redis
This crate use a [docker-compose.yml](redis/docker-compose.yml) to start a redis server. Connects to redis server, sets a key and then fetches value 

### vault
This crate use a [docker-compose.yml](vault/docker-compose.yml) to start a vault server, and tests out setting and getting key/value secrets.
