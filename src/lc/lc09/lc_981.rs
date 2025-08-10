// https://leetcode.com/problems/time-based-key-value-store/
// 981. Time Based Key-Value Store
use std::collections::HashMap;
pub struct TimeMap {
    m: HashMap<String, Vec<(i32, String)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.m.entry(key).or_insert(vec![]).push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.m.get(&key) {
            let p = v.partition_point(|(t, _)| *t <= timestamp);
            if p > 0 {
                return v[p - 1].1.clone();
            }
        }
        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_based_key_value_store() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
    }
}
