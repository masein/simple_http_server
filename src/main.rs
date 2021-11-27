#![allow(dead_code)]
use server::Server;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;

fn main() {
    let defualt_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PIBLIC_PATH").unwrap_or(defualt_path);
    println!("public path: {}", public_path);
    let url = "127.0.0.1:8080";
    let server= Server::new(url.to_string());
    server.run(WebsiteHandler::new(public_path));
}