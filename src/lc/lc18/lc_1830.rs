// https://leetcode.com/problems/minimum-number-of-operations-to-make-string-sorted/
// 1830. Minimum Number of Operations to Make String Sorted
pub struct Solution;
const MOD: i64 = 1000000007;
impl Solution {
    fn div(mut a: i64, mut b: i64) -> i64 {
        while a % b != 0 {
            let t = MOD / b + 1;
            b = (b * t) % MOD;
            a = (a * t) % MOD;
        }
        a / b
    }
    pub fn make_string_sorted(s: String) -> i32 {
        let mut ans = 0;
        let mut cnt = [0; 26];
        let s = s.chars().map(|c| (c as u8 - b'a') as usize).collect::<Vec<_>>();
        let mut min = s[s.len() - 1];
        let mut perm = 1;
        cnt[min] = 1;
        for i in (0..s.len() - 1).rev() {
            let pperm = Self::div(perm, (cnt[s[i]] + 1) as i64);
            if s[i] > min {
                for j in (0..s[i]).rev() {
                    if cnt[j] > 0 {
                        ans = (ans + pperm * cnt[j] % MOD) % MOD;
                    }
                }
            } else if s[i] < min {
                min = s[i];
            }
            cnt[s[i]] += 1;
            perm = pperm * (s.len() - i) as i64 % MOD;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_make_string_sorted() {
        assert_eq!(
            Solution::make_string_sorted("leetcodeleetcodeleetcode".to_string()),
            982157772
        );
        assert_eq!(Solution::make_string_sorted("cba".to_string()), 5);
        assert_eq!(Solution::make_string_sorted("aabaa".to_string()), 2);
    }
}
