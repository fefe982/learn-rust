// https://leetcode.com/problems/count-k-subsequences-of-a-string-with-maximum-beauty/
// 2842. Count K Subsequences of a String With Maximum Beauty
pub struct Solution;
impl Solution {
    pub fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {
        if k > 26 {
            return 0;
        }
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
        }
        cnt.sort_unstable_by(|a, b| b.cmp(a));
        let mut ans = 1;
        let mut c = 1;
        let m = 1_0000_00007;
        let k = k as usize;
        for i in 0..k {
            ans = (ans * cnt[i] as i64) % m;
            if i > 0 && cnt[i] == cnt[i - 1] {
                c += 1;
            } else {
                c = 1;
            }
        }
        let div = |mut a: i64, mut b: i64| -> i64 {
            while a % b != 0 {
                let n = m / b;
                a = a * (n + 1) % m;
                b = b * (n + 1) % m;
            }
            a / b
        };
        for j in k..26 {
            if cnt[j] == cnt[j - 1] {
                c += 1;
                ans = div(ans * c % m, (j - k + 1) as i64);
            } else {
                break;
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_k_subsequences_with_max_beauty() {
        assert_eq!(Solution::count_k_subsequences_with_max_beauty("bcca".to_string(), 2), 4);
        assert_eq!(
            Solution::count_k_subsequences_with_max_beauty("abbcd".to_string(), 4),
            2
        );
    }
}
