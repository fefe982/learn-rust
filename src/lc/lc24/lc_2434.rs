// https://leetcode.com/problems/using-a-robot-to-print-the-lexicographically-smallest-string/
// 2434. Using a Robot to Print the Lexicographically Smallest String
pub struct Solution;
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut min = vec!['z'; s.len()];
        for i in (0..s.len()).rev() {
            min[i] = min.get(i + 1).cloned().unwrap_or('z').min(s[i]);
        }
        let mut res = "".to_string();
        let mut stk = vec![];
        for i in 0..s.len() {
            while !stk.is_empty() && stk[stk.len() - 1] <= min[i] {
                res.push(stk.pop().unwrap());
            }
            stk.push(s[i]);
        }
        while let Some(c) = stk.pop() {
            res.push(c);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn robot_with_string() {
        assert_eq!(Solution::robot_with_string("zza".to_string()), "azz");
        assert_eq!(Solution::robot_with_string("bac".to_string()), "abc");
        assert_eq!(Solution::robot_with_string("bdda".to_string()), "addb");
    }
}
