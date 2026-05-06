// https://leetcode.com/problems/plates-between-candles/
// 2055. Plates Between Candles
pub struct Solution;
impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut pre_sum = vec![0; s.len() + 1];
        let mut left = vec![0; s.len()];
        let mut right = vec![0; s.len()];
        for (i, c) in s.chars().enumerate() {
            pre_sum[i + 1] = pre_sum[i] + if c == '*' { 1 } else { 0 };
            left[i] = if c == '|' {
                i as i32
            } else {
                if i > 0 {
                    left[i - 1]
                } else {
                    -1
                }
            };
        }
        for i in (0..s.len()).rev() {
            right[i] = if s.as_bytes()[i] == b'|' {
                i as i32
            } else {
                if i < s.len() - 1 {
                    right[i + 1]
                } else {
                    -1
                }
            };
        }
        queries
            .into_iter()
            .map(|q| {
                let l = right[q[0] as usize];
                let r = left[q[1] as usize];
                if l != -1 && r != -1 && l < r {
                    pre_sum[r as usize + 1] - pre_sum[l as usize]
                } else {
                    0
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn plates_between_candles() {
        assert_eq!(
            Solution::plates_between_candles("**|**|***|".to_string(), vec_vec![[2, 5], [5, 9]]),
            [2, 3]
        );
        assert_eq!(
            Solution::plates_between_candles(
                "***|**|*****|**||**|*".to_string(),
                vec_vec![[1, 17], [4, 5], [14, 17], [5, 11], [15, 16]]
            ),
            vec![9, 0, 0, 0, 0]
        );
    }
}
