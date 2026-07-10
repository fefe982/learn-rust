// https://leetcode.com/problems/find-the-substring-with-maximum-cost/
// 2606. Find the Substring With Maximum Cost
pub struct Solution;
impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut v = [0; 26];
        for i in 0..26 {
            v[i] = i as i32 + 1;
        }
        for (i, c) in chars.bytes().enumerate() {
            v[(c - b'a') as usize] = vals[i];
        }
        let mut max = 0;
        let mut pre = 0;
        for c in s.bytes() {
            pre = pre.max(0) + v[(c - b'a') as usize];
            max = max.max(pre);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_cost_substring() {
        assert_eq!(
            Solution::maximum_cost_substring("adaa".to_string(), "d".to_string(), vec![-1000]),
            2
        );
        assert_eq!(
            Solution::maximum_cost_substring("abc".to_string(), "abc".to_string(), vec![-1, -1, -1]),
            0
        );
    }
}
