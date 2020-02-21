extern crate dotenv;
extern crate hashicorp_vault as vault;

fn main() {
    dotenv::dotenv().ok();

    let host = "http://localhost:8200";
    let token = "test";
    let client = vault::Client::new(host, token).unwrap();

    let _ = client.set_secret("foo", "bar");

    let secret = client.get_secret("foo").unwrap();

	println!("Secret is \"bar\": {}", secret);
}
