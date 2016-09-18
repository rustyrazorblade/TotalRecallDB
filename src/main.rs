//extern crate db;
//mod database;
mod db;

fn main() {
    println!("Hello, world!");
    let database = db::database::Database::new();


}
