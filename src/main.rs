#![feature(plugin)]
#![feature(test)]

#![plugin(peg_syntax_ext)]

#[macro_use]
extern crate nom;
extern crate clap;
extern crate termion;
extern crate byteorder;
extern crate env_logger;
extern crate vec_map;

#[macro_use]
extern crate log;


mod db;

use db::server::Server;
use db::database::{Database, QueryResult};
use clap::{Arg, App, SubCommand};

use std::io::{self, Read};
use std::io::{Write, stdout, stdin};

use termion::input::TermRead;
use termion::{color, style};


fn main() {
    println!("Starting the worst database ever created!! (exit to exit)");
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
    let mut db = Database::new();
    let mut stdin = stdin();
    let mut stdout = stdout();
    let prompt = "embedded>";

    let _ = env_logger::init();

    loop {

        write!(stdout, "{}[?] {}{} ", color::Fg(color::Green), style::Reset, prompt).unwrap();
        stdout.lock().flush().unwrap();

        match TermRead::read_line(&mut stdin) {
            Ok(Some(buffer)) => {
                if buffer == "exit" {
                    write!(stdout, "Exiting\r\n");
                    stdout.lock().flush().unwrap();
                    return;
                }
                let x= match db.execute(&buffer) {
                    Ok(QueryResult::StreamCreated) =>
                        String::from("Stream Created.\n"),
                    Ok(QueryResult::Insert(id)) =>
                        format!("Inserted {}", id),
                    _ => String::from("Fail?")

                };
                println!("{}", x);
            },
            Ok(None) => {},
            Err(e) => {}
        }
    }

}
