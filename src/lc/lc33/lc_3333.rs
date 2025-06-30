// https://leetcode.com/problems/find-the-original-typed-string-ii/
// 3333. Find the Original Typed String II
pub struct Solution;
impl Solution {
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let k = k as usize;
        let word = word.as_bytes();
        let mut last = b'.';
        let mut streak = 0;
        let m = 1_000_000_007i64;
        let mut dp = vec![1; k + 1];
        let mut total = 1;
        dp[0] = 0;
        let mut seg = 0;
        for &c in word.iter().chain([b','].iter()) {
            if c == last {
                streak += 1;
            } else {
                if streak > 0 {
                    if seg <= k {
                        let mut ndp = vec![0; k + 1];
                        for i in 1..k {
                            let s = i.min(streak as usize);
                            ndp[i + 1] = (ndp[i] + dp[i] - dp[i - s] + m) % m;
                        }
                        dp = ndp;
                    }
                    total = (total * streak) % m;
                    seg += 1;
                }
                streak = 1;
            }
            last = c;
        }
        ((total + m - dp[k]) % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_possible_string_count() {
        assert_eq!(Solution::possible_string_count("aabbccdd".to_string(), 7), 5);
        assert_eq!(Solution::possible_string_count("aabbccdd".to_string(), 8), 1);
        assert_eq!(Solution::possible_string_count("aaabbb".to_string(), 3), 8);
    }
}
