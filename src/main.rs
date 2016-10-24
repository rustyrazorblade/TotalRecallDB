#![feature(plugin)]
#![feature(test)]

#![plugin(peg_syntax_ext)]

#[macro_use]
extern crate nom;
extern crate clap;

mod db;

use db::server::Server;
use clap::{Arg, App, SubCommand};

fn main() {
    println!("Starting the worst database ever created!!");
    let server = Server::new();
    let app = App::new("StreamingDB")
                  .version("v1.0")
                  .author("Jon Haddad, <jon@jonhaddad.com>")
                  .subcommand(SubCommand::with_name("test"))
                  .get_matches();

    if let Some(matches) = app.subcommand_matches("test") {
        run_test_repl();
    }

}

fn run_test_repl() {
    println!("Running test repl");
}
