mod service;
mod utils;

use std::process;

use service::{get_score};
use utils::send_get;

fn main() {
    let url = "https://api.bilibili.com/pgc/review/long/list?media_id=28235358&ps=20&sort=0";
    let resp = match send_get(&url) {
        Ok(res) => res,
        _ => {
            print!("获取评分失败");
            process::exit(-1);
        }
    };
    let score = get_score(&resp);
    println!("{:#?}", score);
}