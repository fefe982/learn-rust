// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
// 1239. Maximum Length of a Concatenated String with Unique Characters
pub struct Solution;
impl Solution {
    fn max_l(arr: &[i32], acc: i32) -> i32 {
        if arr.len() == 0 {
            return acc.count_ones() as i32;
        }
        let mut m = Solution::max_l(&arr[1..], acc);
        if arr[0] & acc == 0 {
            m = m.max(Solution::max_l(&arr[1..], acc | arr[0]));
        }
        m
    }
    pub fn max_length(arr: Vec<String>) -> i32 {
        let arr = arr
            .into_iter()
            .filter_map(|x| {
                let v: i32 = x.as_bytes().into_iter().fold(0, |acc, &c| acc | (1 << (c - b'a')));
                if x.len() == v.count_ones() as usize {
                    Some(v)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Solution::max_l(&arr, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_length() {
        assert_eq!(Solution::max_length(vec_str!["aa", "bb"]), 0);
        assert_eq!(Solution::max_length(vec_str!["un", "iq", "ue"]), 4);
        assert_eq!(Solution::max_length(vec_str!["cha", "r", "act", "ers"]), 6);
        assert_eq!(Solution::max_length(vec_str!["abcdefghijklmnopqrstuvwxyz"]), 26);
    }
}
