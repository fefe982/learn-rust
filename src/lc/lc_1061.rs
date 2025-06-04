// https://leetcode.com/problems/lexicographically-smallest-equivalent-string/
// 1061. Lexicographically Smallest Equivalent String
pub struct Solution;
impl Solution {
    fn get(map: &mut Vec<usize>, c: usize) -> usize {
        if map[c] != c {
            map[c] = Self::get(map, map[c]);
        }
        map[c]
    }
    fn union(map: &mut Vec<usize>, c1: usize, c2: usize) {
        let c1 = Self::get(map, c1);
        let c2 = Self::get(map, c2);
        if c1 != c2 {
            map[c1] = c2.min(c1);
            map[c2] = c2.min(c1);
        }
    }
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut map = (0..26).collect::<Vec<_>>();
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            let c1 = (c1 as u8 - b'a') as usize;
            let c2 = (c2 as u8 - b'a') as usize;
            Self::union(&mut map, c1, c2);
        }
        base_str
            .chars()
            .map(|c| {
                let c = (c as u8 - b'a') as usize;
                (Self::get(&mut map, c) as u8 + b'a') as char
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_equivalent_string() {
        assert_eq!(
            Solution::smallest_equivalent_string("parker".to_string(), "morris".to_string(), "parser".to_string()),
            "makkek"
        );
        assert_eq!(
            Solution::smallest_equivalent_string("hello".to_string(), "world".to_string(), "hold".to_string()),
            "hdld"
        );
        assert_eq!(
            Solution::smallest_equivalent_string(
                "leetcode".to_string(),
                "programs".to_string(),
                "sourcecode".to_string()
            ),
            "aauaaaaada"
        );
    }
}
