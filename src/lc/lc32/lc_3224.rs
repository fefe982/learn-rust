// https://leetcode.com/problems/count-the-number-of-substrings-with-dominant-ones/
// 3224. Count the Number of Substrings With Dominant Ones
pub struct Solution;
impl Solution {
    pub fn count_substr(s: String) -> i32 {
        let mut ans = 0;
        let mut pre = Vec::with_capacity(s.len());
        pre.push(0);
        for (i, c) in s.chars().enumerate() {
            let mut n0 = 0;
            let i = i as i32;
            if c == '0' {
                pre.push(i + 1);
            }
            let e = i + 2;
            let mut c0 = i + 1;
            for &p0 in pre.iter().rev() {
                let ee = e - n0 - n0 * n0;
                if ee >= p0 {
                    ans += ee.min(c0) - p0;
                }
                if ee <= 0 {
                    break;
                }
                c0 = p0;
                n0 += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_substr() {
        assert_eq!(Solution::count_substr("00011".to_string()), 5);
        assert_eq!(Solution::count_substr("101101".to_string()), 16);
    }
}
