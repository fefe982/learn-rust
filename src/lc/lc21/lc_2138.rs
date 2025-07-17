// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/
// 2138. Divide a String Into Groups of Size k
pub struct Solution;
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;
        let mut ret: Vec<String> = Vec::with_capacity(s.len() / k + 1);
        for c in s.chars() {
            if ret.is_empty() || ret.last().unwrap().len() == k {
                ret.push(String::with_capacity(k));
            }
            ret.last_mut().unwrap().push(c);
        }
        let remain = k - ret.last().unwrap().len();
        if remain > 0 {
            ret.last_mut().unwrap().push_str(&fill.to_string().repeat(remain));
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn divide_string() {
        assert_eq!(
            Solution::divide_string("abcdefghi".to_string(), 3, 'x'),
            vec_str!["abc", "def", "ghi"]
        );
        assert_eq!(
            Solution::divide_string("abcdefghij".to_string(), 3, 'x'),
            vec_str!["abc", "def", "ghi", "jxx"]
        );
    }
}
