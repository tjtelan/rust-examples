#![allow(unused)]

extern crate dotenv;
extern crate redis;
use redis::{Connection,Commands,RedisResult};

use std::env;

fn connect_to_redis() -> Connection {
    let redis_key = "REDIS_URI";
    let redis_addr = match env::var_os(redis_key) {
        Some(addr) => {
            println!("Returning the env var value: {:?}", addr);
            let url = format!("{}", addr.into_string().unwrap());
            String::from(url)
        }
        None => {
            println!("Returning default bc env var not set");
            String::from("redis://127.0.0.1:6379")
        }
    };

    let client = redis::Client::open(&redis_addr[..]).unwrap();
    client.get_connection().unwrap()
}

fn set_an_integer(redis_connection : &mut Connection, user_integer : isize) -> RedisResult<()> {
    // connect to redis
    // throw away the result, just make sure it does not fail
    let _: () = redis_connection.set("my_key", user_integer)?;
    Ok(())
}

fn fetch_an_integer(redis_connection : &mut Connection) -> RedisResult<isize> {
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    redis_connection.get("my_key")
}

fn main() {
    // Load env vars from .env file, if exists
    dotenv::dotenv();

    let mut con = connect_to_redis();
    set_an_integer(&mut con, 42);
    println!("{:?}", fetch_an_integer(&mut con));
}
