mod db;

fn main() {
    println!("Hello, world!");
    let server = db::server::Server::new();
}
