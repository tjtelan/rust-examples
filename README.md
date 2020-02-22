# Rust examples

This repo consists of the example Rust binaries I write or copy/pasted. The intentions are to get familiar with using opinionated features of new crates.

This repo is organized into Rust workspaces, but you may need to `cargo run` within a specific crate after following instructions within the crate's README.

**Disclaimer**: The crates I use in these examples do not necessarily represent a recommendation.

## Description of example crates
### cli-clap
This directly uses the `clap` crate using an external yaml to define the cli schema.

### cli_grpc_diesel
** Not currently compiling via workspace**

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

### redis
This crate use a [docker-compose.yml](redis/docker-compose.yml) to start a redis server. Connects to redis server, sets a key and then fetches value 

### vault
This crate use a [docker-compose.yml](vault/docker-compose.yml) to start a vault server. Uses my personal fork of `hashicorp_vault`, and tests out setting and getting key/value secrets.
