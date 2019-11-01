use std::io::{self, Write};
use hyper::{Client, rt::{self, Future, Stream}};

fn main() {
    hyper_client();
}

fn hyper_client() {
    rt::run(
        rt::lazy(|| {
            // The `lazy` is because we don't want any of this executing *right now*,
            // but rather once the runtime has started up all its resources.
            let client = Client::new();
            let uri = "http://httpbin.org/ip".parse().unwrap();

            client
                .get(uri)
                .map(|response|{
                    println!("Response: {}", response.status());
                })
                .map_err(|err|{
                    println!("Error: {}", err);
                })
        }))
}

fn hyper_server() {

}
