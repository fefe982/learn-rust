// https://leetcode.com/problems/substring-xor-queries/
// 2564. Substring XOR Queries
pub struct Solution;
impl Solution {
    pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = std::collections::HashMap::new();
        let s = s.as_bytes();
        for i in 0..s.len() {
            if s[i] == b'0' {
                if map.get(&0).is_none() {
                    map.insert(0, (i as i32, i as i32));
                }
                continue;
            }
            let mut num = 0;
            for j in i..s.len().min(i + 30) {
                num = (num << 1) | (s[j] - b'0') as i32;
                if map.get(&num).is_none() {
                    map.insert(num, (i as i32, j as i32));
                }
            }
        }
        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let (l, r) = (query[0], query[1]);
            if let Some(&(i, j)) = map.get(&(l ^ r)) {
                result.push(vec![i, j]);
            } else {
                result.push(vec![-1, -1]);
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn substring_xor_queries() {
        assert_eq!(
            Solution::substring_xor_queries("101101".to_string(), vec_vec![[0, 5], [1, 2]]),
            vec_vec![[0, 2], [2, 3]]
        );
        assert_eq!(
            Solution::substring_xor_queries("0101".to_string(), vec_vec![[12, 8]]),
            vec_vec![[-1, -1]]
        );
        assert_eq!(
            Solution::substring_xor_queries("111".to_string(), vec_vec![[4, 5]]),
            vec_vec![[0, 0]]
        );
    }
}
