use std::{process, default};

use indicatif::ProgressBar;

use crate::utils::{read_json, send_get};

pub struct CommentCount {
    pub zero_score: u64,
    pub one_score: u64,
    pub two_score: u64,
    pub three_score: u64,
    pub four_score: u64,
    pub five_score: u64,
}

impl Default for CommentCount {
    fn default() -> CommentCount {
        CommentCount {
            zero_score: 0,
            one_score: 0,
            two_score: 0,
            three_score: 0,
            four_score: 0,
            five_score: 0,
        }
    }
}

pub fn get_full_comment_count(media_id: &str) -> CommentCount {
    println!("正在获取长评。。。");
    get_long_comment_count(media_id);
    println!("正在获取短评。。。");
    let comment_count = get_short_comment_count(media_id);
    comment_count
}

fn get_long_comment_count(media_id: &str) -> CommentCount {
    let long_url = format!(
        "{}{}{}",
        "https://api.bilibili.com/pgc/review/long/list?media_id=", media_id, "&ps=20&sort=0"
    );
    return get_comment_count(&long_url);
}

fn get_short_comment_count(media_id: &str) -> CommentCount {
    let short_url = format!(
        "{}{}{}",
        "https://api.bilibili.com/pgc/review/short/list?media_id=", media_id, "&ps=20&sort=0"
    );
    return get_comment_count(&short_url);
}

fn get_comment_count(url: &str) -> CommentCount {
    let mut comment_count = CommentCount {
        zero_score: 0,
        one_score: 0,
        two_score: 0,
        three_score: 0,
        four_score: 0,
        five_score: 0,
    };
    let json = match send_get(url) {
        Ok(res) => read_json(&res),
        Err(_) => {
            print!("获取评分信息错误");
            process::exit(-1);
        }
    };
    //获取 next id 除第一次评论请求外其余请求均需要此序号
    let mut next_id = json["data"]["next"].to_string();
    //获取评论总个数
    let total = json["data"]["total"].as_u64().unwrap();
    //实列化进度条
    let pb = ProgressBar::new(total);
    for e in json["data"]["list"].as_array().unwrap() {
        let num = e["score"].as_u64().unwrap();
        update_comment_count(num, &mut comment_count);
        pb.inc(1);
    }
    loop {
        let second_url = format!("{}{}{}", url, "&cursor=", next_id);
        let temp = match send_get(&second_url) {
            Ok(res) => read_json(&res),
            Err(_) => {
                print!("获取评分信息错误");
                process::exit(-1);
            }
        };
        for e in temp["data"]["list"].as_array().unwrap() {
            let num = e["score"].as_u64().unwrap();
            update_comment_count(num, &mut comment_count);
            pb.inc(1);
        }
        next_id = temp["data"]["next"].to_string();
        if next_id == "0" {
            //完成进度条绘制
            pb.finish_with_message("完成");
            break;
        }
    }
    comment_count
}

fn update_comment_count(num : u64, comment_count:&mut CommentCount) {
    match num {
        2 => comment_count.one_score += 1,
        4 => comment_count.two_score += 1,
        6 => comment_count.three_score += 1,
        8 => comment_count.four_score += 1,
        10 => comment_count.five_score += 1,
        0 => comment_count.zero_score += 1,
        _ => (),
    }
}
