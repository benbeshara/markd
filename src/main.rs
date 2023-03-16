mod md_parse;
mod web;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    web::main();
}
