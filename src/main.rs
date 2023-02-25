mod service;
mod utils;

use service::get_score;
use service::get_full_json;

fn main() {
    let full_json = get_full_json("28235358");
    let score = get_score(full_json);
    println!("{:#?}",score);
}