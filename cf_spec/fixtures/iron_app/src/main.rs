extern crate iron;

use iron::prelude::*;
use iron::status;
use std::env;

fn main() {
    let key = "PORT";

    let port: String = match env::var(key) {
        Ok(val) => val,
        Err(_) => String::from("8080"),
    };

    let http_url = format!("0.0.0.0:{}", port);
    println!("Booting up Hello World Iron app at http://{}", http_url);

    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World from the Iron web framework!")))
    }).http(&*http_url).unwrap();
}

