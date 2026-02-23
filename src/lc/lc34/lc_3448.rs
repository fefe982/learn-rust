// https://leetcode.com/problems/count-substrings-divisible-by-last-digit/
// 3448. Count Substrings Divisible by Last Digit
pub struct Solution;
impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let mut cnt = [[0; 10]; 10];
        let mut ans = 0;
        for c in s.chars() {
            for i in 1..10 {
                cnt[i][0] += 1;
            }
            let d = (c as u8 - '0' as u8) as usize;
            let mut ncnt = [[0; 10]; 10];
            for i in 1..10 {
                for j in 0..i {
                    ncnt[i][(j * 10 + d) % i] += cnt[i][j];
                }
            }
            cnt = ncnt;
            if d != 0 {
                ans += cnt[d][0];
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_substrings() {
        assert_eq!(Solution::count_substrings("12936".to_string()), 11);
        assert_eq!(Solution::count_substrings("5701283".to_string()), 18);
        assert_eq!(Solution::count_substrings("1010101010".to_string()), 25);
    }
}
