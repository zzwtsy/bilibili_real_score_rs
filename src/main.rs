use std::io;
use std::process;
use std::time::Instant;

use crate::service::get_full_comment_count;

mod service;
mod utils;

fn main() {
    loop {
        println!("1.计算真实评分");
        println!("0.退出程序");
        println!("请输入对应数字:");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let i: i32 = choice.trim().parse().expect("choice to i32 err");
        match i {
            1 => run(),
            0 => {
                process::exit(0);
            }
            _ => println!("没有此选项"),
        }
    }
}

fn run() {
    println!("请输入 media_id:");
    let mut media_id = String::new();
    io::stdin()
        .read_line(&mut media_id)
        .expect("Failed to read input");
    let start = Instant::now();
    let full_comment_count = get_full_comment_count(media_id.as_str());
    let zero_score = full_comment_count.get(0).unwrap();
    let one_score = full_comment_count.get(1).unwrap();
    let two_score = full_comment_count.get(2).unwrap();
    let three_score = full_comment_count.get(3).unwrap();
    let four_score = full_comment_count.get(4).unwrap();
    let five_score = full_comment_count.get(5).unwrap();
    let total_score: f64 =
        ((one_score + (two_score * 2) + (three_score * 3) + (four_score * 4) + (five_score * 5))
            * 2) as f64;
    let total_comment: f64 = (one_score + two_score + three_score + four_score + five_score) as f64;
    let real_score: f64 = (total_score / total_comment) as f64;
    println!(
        "零分人数：{}\n一分人数：{}\n二分人数：{}\n三分人数：{}\n四分人数：{}\n五分人数：{}",
        zero_score, one_score, two_score, three_score, four_score, five_score
    );
    println!("真实评分为：{:.1}", real_score);
    let duration = start.elapsed();
    println!("本次计算时间为：{:?}",duration);
}
