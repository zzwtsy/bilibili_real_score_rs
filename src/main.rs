use std::io;

use service::get_full_json;
use service::get_score;

mod service;
mod utils;

fn main() {
    println!("请输入 media_id:");
    let mut media_id = String::new();
    io::stdin().read_line(&mut media_id).expect("Failed to read input");
    let full_json = get_full_json(media_id.as_str());
    let score = get_score(full_json);
    let zero_score = score.get(0).unwrap();
    let one_score = score.get(1).unwrap();
    let two_score = score.get(2).unwrap();
    let three_score = score.get(3).unwrap();
    let four_score = score.get(4).unwrap();
    let five_score = score.get(5).unwrap();
    let real_score = (one_score + (two_score * 2 + three_score * 3 + four_score * 4 + five_score * 5)) / (one_score + two_score + three_score + four_score + five_score);
    println!("零分人数：{}\n一分人数：{}\n二分人数：{}\n三分人数：{}\n四分人数：{}\n五分人数：{}", zero_score, one_score, two_score, three_score, four_score, five_score);
    println!("真实评分为：{}", real_score);
}