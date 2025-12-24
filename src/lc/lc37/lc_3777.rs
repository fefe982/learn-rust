// https://leetcode.com/problems/minimum-deletions-to-make-alternating-substring/
// 3777. Minimum Deletions to Make Alternating Substring
pub struct Solution;
impl Solution {
    pub fn min_deletions(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut s = s.as_bytes().into_iter().map(|&c| c - b'A').collect::<Vec<_>>();
        let mut alt = vec![0; s.len()];
        fn add(v: &mut Vec<i32>, idx: usize, val: i32) {
            let mut idx = idx;
            while idx < v.len() {
                v[idx] += val;
                idx += idx & (!idx + 1);
            }
        }
        fn get(v: &Vec<i32>, idx: usize) -> i32 {
            let mut sum = 0;
            let mut idx = idx;
            while idx > 0 {
                sum += v[idx];
                idx -= idx & (!idx + 1);
            }
            sum
        }
        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                add(&mut alt, i, 1);
            }
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            if q[0] == 1 {
                let idx = q[1] as usize;
                if idx > 0 {
                    let v = if s[idx] == s[idx - 1] { 1 } else { -1 };
                    add(&mut alt, idx, v);
                }
                if idx < s.len() - 1 {
                    let v = if s[idx] == s[idx + 1] { 1 } else { -1 };
                    add(&mut alt, idx + 1, v);
                }
                s[idx] = 1 - s[idx];
            } else {
                let l = q[1] as usize;
                let r = q[2] as usize;
                ans.push(q[2] - q[1] - get(&alt, r) + get(&alt, l));
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_deletions() {
        assert_eq!(
            Solution::min_deletions("ABA".to_string(), vec_vec![[2, 1, 2], [1, 1], [2, 0, 2]]),
            [0, 2]
        );
        assert_eq!(
            Solution::min_deletions("ABB".to_string(), vec_vec![[2, 0, 2], [1, 2], [2, 0, 2]]),
            [1, 0]
        );
        assert_eq!(
            Solution::min_deletions("BABA".to_string(), vec_vec![[2, 0, 3], [1, 1], [2, 1, 3]]),
            [0, 1]
        );
    }
}
