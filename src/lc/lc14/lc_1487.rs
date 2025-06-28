// https://leetcode.com/problems/making-file-names-unique/
// 1487. Making File Names Unique
use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut vec: Vec<String> = Vec::with_capacity(names.len());
        let mut name_map: HashMap<String, i32> = HashMap::new();
        for name in names {
            let mut new_name: String;
            let mut cnt;
            if let Some(cnt_) = name_map.get(&name) {
                cnt = *cnt_;
            } else {
                name_map.insert(name.clone(), 0);
                vec.push(name.clone());
                continue;
            }
            loop {
                cnt += 1;
                new_name = format!("{}({})", name, cnt);
                if let None = name_map.get(&new_name) {
                    name_map.insert(new_name.clone(), 0);
                    vec.push(new_name);
                    break;
                }
            }
            *name_map.get_mut(&name).unwrap() = cnt;
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_folder_names() {
        assert_eq!(
            Solution::get_folder_names(vec![
                String::from("pes"),
                String::from("fifa"),
                String::from("gta"),
                String::from("pes(2019)")
            ]),
            vec![
                String::from("pes"),
                String::from("fifa"),
                String::from("gta"),
                String::from("pes(2019)")
            ]
        );
        assert_eq!(
            Solution::get_folder_names(vec![
                String::from("gta"),
                String::from("gta(1)"),
                String::from("gta"),
                String::from("avalon")
            ]),
            vec![
                String::from("gta"),
                String::from("gta(1)"),
                String::from("gta(2)"),
                String::from("avalon")
            ]
        );
        assert_eq!(
            Solution::get_folder_names(vec![
                String::from("onepiece"),
                String::from("onepiece(1)"),
                String::from("onepiece(2)"),
                String::from("onepiece(3)"),
                String::from("onepiece")
            ]),
            vec![
                String::from("onepiece"),
                String::from("onepiece(1)"),
                String::from("onepiece(2)"),
                String::from("onepiece(3)"),
                String::from("onepiece(4)")
            ]
        );
    }
}
