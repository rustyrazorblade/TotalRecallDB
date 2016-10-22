#![feature(plugin)]
#![feature(test)]

#![plugin(peg_syntax_ext)]

#[macro_use]
extern crate nom;

mod db;

use db::server::Server;

fn main() {
    println!("Hello, world!");
    let server = Server::new();
}
