use dialoguer::Input;

use crate::service::get_full_comment_count;
use std::io;
use std::process;
use std::time::Instant;

mod service;
mod utils;

fn main() -> io::Result<()> {
    loop {
        println!("1.计算真实评分");
        println!("0.退出程序");
        let input: i32 = Input::new().with_prompt("请输入对应数字").interact_text()?;
        match input {
            1 => {
                let media_id: String= Input::new().with_prompt("请输入 media_id").interact_text()?;
                run(&media_id);
            },
            0 => {
                process::exit(0);
            }
            _ => println!("没有此选项"),
        }
    }
}

fn run(media_id: &str) {
    let start = Instant::now();
    let full_comment_count = get_full_comment_count(media_id);
    let zero_score = full_comment_count.zero_score;
    let one_score = full_comment_count.one_score;
    let two_score = full_comment_count.two_score;
    let three_score = full_comment_count.three_score;
    let four_score = full_comment_count.four_score;
    let five_score = full_comment_count.five_score;
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
    println!("本次计算时间为：{:.0?}", duration);
    println!("====================");
}
