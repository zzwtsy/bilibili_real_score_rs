use std::process;

use indicatif::ProgressBar;

use crate::utils::{read_json, send_get};

pub fn get_full_comment_count(media_id: &str) -> [i32; 6] {
    println!("正在获取长评。。。");
    let long_comments_count = get_long_comment_count(media_id);
    println!("正在获取短评。。。");
    let short_comments_count = get_short_comment_count(media_id);
    let score_count = [
        long_comments_count.get(0).unwrap() + short_comments_count.get(0).unwrap(),
        long_comments_count.get(1).unwrap() + short_comments_count.get(1).unwrap(),
        long_comments_count.get(2).unwrap() + short_comments_count.get(2).unwrap(),
        long_comments_count.get(3).unwrap() + short_comments_count.get(3).unwrap(),
        long_comments_count.get(4).unwrap() + short_comments_count.get(4).unwrap(),
        long_comments_count.get(5).unwrap() + short_comments_count.get(5).unwrap(),
    ];
    score_count
}

fn get_long_comment_count(media_id: &str) -> [i32; 6] {
    let long_url = format!(
        "{}{}{}",
        "https://api.bilibili.com/pgc/review/long/list?media_id=", media_id, "&ps=20&sort=0"
    );
    return get_comment_count(&long_url);
}

fn get_short_comment_count(media_id: &str) -> [i32; 6] {
    let short_url = format!(
        "{}{}{}",
        "https://api.bilibili.com/pgc/review/short/list?media_id=", media_id, "&ps=20&sort=0"
    );
    return get_comment_count(&short_url);
}

fn get_comment_count(url: &str) -> [i32; 6] {
    let mut zero_score = 0;
    let mut one_score = 0;
    let mut two_score = 0;
    let mut three_score = 0;
    let mut four_score = 0;
    let mut five_score = 0;
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
        match num {
            2 => one_score += 1,
            4 => two_score += 1,
            6 => three_score += 1,
            8 => four_score += 1,
            10 => five_score += 1,
            0 => zero_score += 1,
            _ => (),
        }
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
            match num {
                2 => one_score += 1,
                4 => two_score += 1,
                6 => three_score += 1,
                8 => four_score += 1,
                10 => five_score += 1,
                0 => zero_score += 1,
                _ => (),
            }
            pb.inc(1);
        }
        next_id = temp["data"]["next"].to_string();
        if next_id == "0" {
            //完成进度条绘制
            pb.finish_with_message("完成");
            break;
        }
    }
    let score_info = [
        zero_score,
        one_score,
        two_score,
        three_score,
        four_score,
        five_score,
    ];
    score_info
}
