// https://leetcode.com/problems/minimum-number-of-food-buckets-to-feed-the-hamsters/
// 2086. Minimum Number of Food Buckets to Feed the Hamsters
pub struct Solution;
impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        let mut hamsters = hamsters.into_bytes();
        let n = hamsters.len();
        let mut ans = 0;

        for i in 0..n {
            if hamsters[i] == b'H' {
                if i > 0 && hamsters[i - 1] == b'B' {
                    continue;
                }
                if i + 1 < n && hamsters[i + 1] == b'.' {
                    hamsters[i + 1] = b'B';
                    ans += 1;
                } else if i > 0 && hamsters[i - 1] == b'.' {
                    hamsters[i - 1] = b'B';
                    ans += 1;
                } else {
                    return -1;
                }
            }
        }

        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_buckets() {
        assert_eq!(Solution::minimum_buckets("H..H".to_string()), 2);
        assert_eq!(Solution::minimum_buckets(".H.H.".to_string()), 1);
        assert_eq!(Solution::minimum_buckets(".HHH.".to_string()), -1);
    }
}
