#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod db;

fn main() {
    println!("Hello, world!");
    let server = db::server::Server::new();
}
