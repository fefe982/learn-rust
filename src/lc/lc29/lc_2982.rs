// https://leetcode.cn/problems/find-longest-special-substring-that-occurs-thrice-ii/
// 2982. Find Longest Special Substring That Occurs Thrice II
pub struct Solution;
impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut cnt = vec![vec![0; 3]; 26];
        let s = s.as_bytes();
        let mut last = 0;
        let mut cur = 0;
        for &c in s.iter().chain([b'0'].iter()) {
            if c != last && last != 0 {
                let idx = last as usize - 'a' as usize;
                for i in 0..3 {
                    if cur > cnt[idx][i] {
                        let tmp = cnt[idx][i];
                        cnt[idx][i] = cur;
                        cur = tmp;
                    }
                }
                cur = 1;
            } else {
                cur += 1;
            }
            last = c;
        }
        let mut max = -1;
        for i in 0..26 {
            let mut cur = 0;
            if cnt[i][0] == 0 {
                continue;
            }
            if cnt[i][0] > 2 {
                cur = cur.max(cnt[i][0] - 2);
            }
            if cnt[i][1] == cnt[i][0] {
                if cnt[i][1] > 1 {
                    cur = cur.max(cnt[i][1] - 1);
                }
            } else if cnt[i][0] > 0 {
                cur = cur.max(cnt[i][1]);
            }
            if cnt[i][2] > 0 {
                cur = cur.max(cnt[i][2]);
            }
            if cur > 0 {
                max = max.max(cur);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_length() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
        assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
        assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
    }
}
