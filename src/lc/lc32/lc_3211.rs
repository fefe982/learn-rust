// https://leetcode.cn/problems/generate-binary-strings-without-adjacent-zeros/
// 3211. Generate Binary Strings Without Adjacent Zeros
pub struct Solution;
impl Solution {
    fn gen(mut s: String, n: i32, bit: i32, v: &mut Vec<String>) {
        if n == 0 {
            v.push(s);
            return;
        }
        if bit == 1 {
            s.push('0');
            Self::gen(s.clone(), n - 1, 0, v);
            s.pop();
        }
        s.push('1');
        Self::gen(s, n - 1, 1, v);
    }
    pub fn valid_strings(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::gen(String::new(), n, 1, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_strings() {
        assert_eq!(Solution::valid_strings(3), vec!["010", "011", "101", "110", "111"]);
        assert_eq!(Solution::valid_strings(1), vec!["0", "1"]);
    }
}
