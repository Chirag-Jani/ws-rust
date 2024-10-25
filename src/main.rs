mod db;
mod schema;
use db::connect_and_setup;

fn main() {
    let _ = connect_and_setup().unwrap();
}
