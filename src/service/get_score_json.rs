use std::process;

use serde_json::Value;

use crate::utils::{read_json, send_get};

pub fn get_full_json(media_id: &str) -> Vec<Value> {
    let mut vec: Vec<Value> = Vec::new();
    vec.append(&mut get_long_comment(media_id));
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
    let url = format!(
        "{}{}{}",
        "https://api.bilibili.com/pgc/review/short/list?media_id=", media_id, "&ps=20&sort=0"
    );
    return get_comment(url.as_str());
}

fn get_comment(url: &str) -> Vec<Value> {
    let mut vec: Vec<Value> = Vec::new();
    let temp_json = match send_get(url) {
        Ok(res) => read_json(&res),
        _ => {
            print!("获取评分信息错误");
            process::exit(-1);
        }
    };
    vec.push(temp_json["data"]["list"].clone());
    let mut next_id = temp_json["data"]["next"].to_string();
    loop {
        let url1 = format!("{}{}{}", url, "&cursor=", next_id);
        let temp = match send_get(&url1) {
            Ok(res) => read_json(&res),
            Err(_) => {
                // print!("获取评分信息错误{:#?}",vec);
                break;
                // process::exit(-1);
            }
        };
        vec.push(temp["data"]["list"].clone());
        next_id = temp["data"]["next"].to_string();
        if next_id == "0" {
            break;
        }
    }
    return vec;
}
