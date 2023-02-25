use serde_json::Value;

///## 读取 json
pub fn read_json(raw_json: &str) -> Value {
    let parsed = serde_json::from_str(raw_json).unwrap();
    parsed
}