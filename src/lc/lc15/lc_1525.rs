// https://leetcode.com/problems/number-of-good-ways-to-split-a-string/
// 1525. Number of Good Ways to Split a String
pub struct Solution;
impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        let mut cr = 0;
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            cnt[idx] += 1;
            if cnt[idx] == 1 {
                cr += 1;
            }
        }
        let mut ans = 0;
        let mut cl = 0;
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            if cnt[idx] > 0 {
                cl += 1;
                cnt[idx] = -cnt[idx];
            }
            if cnt[idx] == -1 {
                cr -= 1;
            }
            cnt[idx] += 1;
            if cl == cr {
                ans += 1;
            } else if cl > cr {
                break;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_splits() {
        assert_eq!(Solution::num_splits("aacaba".to_string()), 2);
        assert_eq!(Solution::num_splits("abcd".to_string()), 1);
    }
}
