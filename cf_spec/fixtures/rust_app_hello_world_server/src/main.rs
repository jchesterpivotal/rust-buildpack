#![deny(warnings)]
extern crate hyper;
extern crate env_logger;

use hyper::server::{Request, Response};
use std::env;

static PHRASE: &'static [u8] = b"Hello World!";

fn hello(_: Request, res: Response) {
    res.send(PHRASE).unwrap();
}

fn main() {
    env_logger::init().unwrap();
    let key = "PORT";

    let port: String = match env::var(key) {
        Ok(val) => val,
        Err(_) => String::from("8080"),
    };

    let http_url = format!("0.0.0.0:{}", port);
    let _listening = hyper::Server::http(&*http_url).unwrap()
        .handle(hello);
    println!("Listening on http://0.0.0.0:{}", port);
}
