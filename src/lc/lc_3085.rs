// https://leetcode.com/problems/minimum-deletions-to-make-string-k-special/
// 3085. Minimum Deletions to Make String K-Special
pub struct Solution;
impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut cnt = [0; 26];
        let mut tot = 0;
        for c in word.chars() {
            cnt[(c as u8 - b'a') as usize] += 1;
            tot += 1;
        }
        cnt.sort_unstable();
        let mut min = i32::MAX;
        let mut i = 25;
        let mut j = 26;
        let mut last = 100000;
        let mut rj = 0;
        let mut ri = 0;
        loop {
            if cnt[i] == 0 {
                break;
            }
            ri += cnt[i];
            let upper = last.min(cnt[i] + k);
            rj += (last - upper) * (26 - j as i32);
            while cnt[j - 1] > upper {
                j -= 1;
                rj += cnt[j] - upper;
            }
            last = upper;
            min = min.min(tot - ri + rj);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        min
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_deletions() {
        assert_eq!(Solution::minimum_deletions("aabcaba".to_string(), 0), 3);
        assert_eq!(Solution::minimum_deletions("dabdcbdcdcd".to_string(), 2), 2);
        assert_eq!(Solution::minimum_deletions("aaabaaa".to_string(), 2), 1);
    }
}
