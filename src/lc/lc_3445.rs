// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-ii/
// 3445. Maximum Difference Between Even and Odd Frequency II
pub struct Solution;
impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let mut res = i32::MIN;
        let len = s.len();
        let mut cnt = vec![[0; 5]; len + 1];
        for (i, c) in s.as_bytes().iter().enumerate() {
            cnt[i + 1] = cnt[i];
            cnt[i + 1][(c - b'0') as usize] += 1;
        }
        let k = k as usize;
        for a in 0..=4 {
            for b in 0..=4 {
                if a == b {
                    continue;
                }
                let mut prev = [100000; 4];
                let mut j = 0;
                for i in k..=len {
                    while j <= i - k {
                        if cnt[j][b] == cnt[i][b] {
                            break;
                        }
                        let p = ((cnt[j][a] % 2) * 2 + cnt[j][b] % 2) as usize;
                        prev[p] = prev[p].min(cnt[j][a] - cnt[j][b]);
                        j += 1;
                    }
                    let c = ((cnt[i][a] % 2) * 2 + cnt[i][b] % 2) as usize;
                    res = res.max(cnt[i][a] - cnt[i][b] - prev[c ^ 0b10]);
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_difference() {
        assert_eq!(Solution::max_difference("12233".to_string(), 4), -1);
        assert_eq!(Solution::max_difference("1122211".to_string(), 3), 1);
        assert_eq!(Solution::max_difference("110".to_string(), 3), -1);
    }
}
