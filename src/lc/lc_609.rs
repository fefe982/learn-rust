// https://leetcode.com/problems/find-duplicate-file-in-system/
// 609. Find Duplicate File in System
pub struct Solution;
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        for path in paths {
            let mut iter = path.split_ascii_whitespace();
            let dir = iter.next().unwrap();
            while let Some(file) = iter.next() {
                let mut iter = file.split('(');
                let name = iter.next().unwrap();
                let content = iter.next().unwrap().strip_suffix(')').unwrap();
                map.entry(content.to_string())
                    .or_insert(vec![])
                    .push(format!("{}/{}", dir, name));
            }
        }
        map.into_iter()
            .filter_map(|(_, v)| if v.len() > 1 { Some(v) } else { None })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_duplicate() {
        assert_sort_eq!(
            Solution::find_duplicate(vec_str![
                "root/a 1.txt(abcd) 2.txt(efgh)",
                "root/c 3.txt(abcd)",
                "root/c/d 4.txt(efgh)",
                "root 4.txt(efgh)"
            ]),
            vec_vec_str![
                ["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
                ["root/a/1.txt", "root/c/3.txt"]
            ]
        );
    }
}
