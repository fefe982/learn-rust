// https://leetcode.com/problems/similar-string-groups/
// 839. Similar String Groups
pub struct Solution;
impl Solution {
    fn similar(a: &[u8], b: &[u8]) -> bool {
        let mut diff = 0;
        for (&c1, &c2) in a.iter().zip(b.iter()) {
            if c1 != c2 {
                diff += 1;
                if diff > 2 {
                    return false;
                }
            }
        }
        true
    }
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut visited = vec![false; strs.len()];
        let mut cnt = 0;
        let mut stack = vec![];
        for idx in 0..strs.len() {
            if visited[idx] {
                continue;
            }
            cnt += 1;
            stack.push(idx);
            while !stack.is_empty() {
                let i = stack.pop().unwrap();
                if visited[i] {
                    continue;
                }
                visited[i] = true;
                for j in idx + 1..strs.len() {
                    if visited[j] {
                        continue;
                    }
                    if Self::similar(strs[i].as_bytes(), strs[j].as_bytes()) {
                        stack.push(j);
                    }
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;
    #[test]
    fn num_similar_groups() {
        assert_eq!(
            Solution::num_similar_groups(vec_str!["tars", "rats", "arts", "star"]),
            2
        );
        assert_eq!(Solution::num_similar_groups(vec_str!["omv", "ovm"]), 1);
    }
}
