use std::process;

use indicatif::ProgressBar;
use serde_json::Value;

use crate::utils::{read_json, send_get};

pub fn get_full_json(media_id: &str) -> Vec<Value> {
    let mut vec: Vec<Value> = Vec::new();
    println!("正在获取长评。。。");
    vec.append(&mut get_long_comment(media_id));
    println!("正在获取短评。。。");
    vec.append(&mut get_short_comment(media_id));
    vec
}

fn get_long_comment(media_id: &str) -> Vec<Value> {
    let url = format!(
        "{}{}{}",
        "https://api.bilibili.com/pgc/review/long/list?media_id=", media_id, "&ps=20&sort=0"
    );
    return get_comment(url.as_str());
}

fn get_short_comment(media_id: &str) -> Vec<Value> {
    let first_url = format!(
        "{}{}{}",
        "https://api.bilibili.com/pgc/review/short/list?media_id=", media_id, "&ps=20&sort=0"
    );
    return get_comment(first_url.as_str());
}

fn get_comment(url: &str) -> Vec<Value> {
    let mut vec: Vec<Value> = Vec::new();
    let json = match send_get(url) {
        Ok(res) => read_json(&res),
        Err(_) => {
            print!("获取评分信息错误");
            process::exit(-1);
        }
    };
    let mut next_id = json["data"]["next"].to_string();
    //获取评论总个数
    let total = json["data"]["total"].as_u64().unwrap();
    //实列化进度条
    let pb = ProgressBar::new(total);
    loop {
        let second_url = format!("{}{}{}", url, "&cursor=", next_id);
        let temp = match send_get(&second_url) {
            Ok(res) => read_json(&res),
            Err(_) => {
                print!("获取评分信息错误");
                process::exit(-1);
            }
        };
        for x in temp["data"]["list"].as_array().unwrap() {
            vec.push(x.clone());
            pb.inc(1);
        };
        next_id = temp["data"]["next"].to_string();
        if next_id == "0" {
            //完成进度条绘制
            pb.finish_with_message("完成");
            break;
        }
    }
    return vec;
}
