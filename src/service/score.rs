use serde_json::Value;

pub fn get_score(vec: Vec<Value>) -> [i64; 6] {
    let mut zero_score = 0;
    let mut one_score = 0;
    let mut two_score = 0;
    let mut three_score = 0;
    let mut four_score = 0;
    let mut five_score = 0;
    for e in vec {
        let num = match e["score"].as_u64() {
            Some(i) => i,
            None => continue
        };
        match num {
            2 => one_score += 1,
            4 => two_score += 1,
            6 => three_score += 1,
            8 => four_score += 1,
            10 => five_score += 1,
            0 => zero_score += 1,
            _ => (),
        }
    }
    let score_info = [zero_score, one_score, two_score, three_score, four_score, five_score];
    score_info
}
