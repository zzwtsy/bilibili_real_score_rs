use serde_json::Value;

pub fn get_score(vec: Vec<Value>) -> [i64; 5]{
    let mut one_score: i64 = 0;
    let mut two_score: i64 = 0;
    let mut three_score: i64 = 0;
    let mut four_score: i64 = 0;
    let mut five_score: i64 = 0;
    for e in vec.iter() {
        let num: i64 = e["score"].to_string().parse().expect("score to i64 err");
        match num {
            2 => one_score = one_score + 1,
            4 => two_score = two_score + 1,
            6 => three_score = three_score + 1,
            8 => four_score = four_score + 1,
            10 => five_score = five_score + 1,
            _ => (),
        }
    }
    let score_info = [one_score,two_score,three_score,four_score,five_score];
    score_info
}
