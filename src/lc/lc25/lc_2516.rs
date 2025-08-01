// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/
// 2516. Take K of Each Character from Left and Right
pub struct Solution;
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut cnt = vec![0; 3];
        let s = s.as_bytes();
        let mut l = 0;
        for i in 0..s.len() {
            cnt[(s[i] - b'a') as usize] += 1;
            if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
                l = i + 1;
                break;
            }
        }
        if cnt[0] < k || cnt[1] < k || cnt[2] < k {
            return -1;
        }
        let mut ans = cnt[0] + cnt[1] + cnt[2];
        let mut r = s.len();
        loop {
            while l > 0 && cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
                cnt[(s[l - 1] - b'a') as usize] -= 1;
                l -= 1;
            }
            if cnt[0] >= k && cnt[1] >= k && cnt[2] >= k {
                ans = ans.min((s.len() - r) as i32);
                break;
            } else {
                ans = ans.min((s.len() - r + l + 1) as i32);
            }
            while r > 0 && (cnt[0] < k || cnt[1] < k || cnt[2] < k) {
                cnt[(s[r - 1] - b'a') as usize] += 1;
                r -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn take_characters() {
        assert_eq!(Solution::take_characters("aabaaaacaabc".to_string(), 2), 8);
        assert_eq!(Solution::take_characters("a".to_string(), 1), -1);
    }
}
