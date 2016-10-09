#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod db;

use db::server::Server;

fn main() {
    println!("Hello, world!");
    let server = Server::new();
}
