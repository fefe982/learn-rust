// https://leetcode.com/problems/minimum-steps-to-convert-string-with-operations/
// 3579. Minimum Steps to Convert String with Operations
pub struct Solution;
impl Solution {
    pub fn min_operations(word1: String, word2: String) -> i32 {
        let word1 = word1
            .as_bytes()
            .iter()
            .map(|&c| (c - b'a') as usize)
            .collect::<Vec<_>>();
        let word2 = word2
            .as_bytes()
            .iter()
            .map(|&c| (c - b'a') as usize)
            .collect::<Vec<_>>();
        let len = word1.len();
        let mut op = vec![vec![0; len]; len];
        let count = |i1: usize, i2: usize, cnt: &mut [[i32; 26]; 26]| -> i32 {
            if word1[i1] == word2[i2] {
                0
            } else if cnt[word2[i2]][word1[i1]] > 0 {
                cnt[word2[i2]][word1[i1]] -= 1;
                0
            } else {
                cnt[word1[i1]][word2[i2]] += 1;
                1
            }
        };
        for i in 0..len {
            let mut cnt = [[[0; 26]; 26]; 4];
            let mut cop = [0, 1, 0, 1];
            for j in 0..len {
                if j > i || i + j >= len {
                    break;
                }
                if j == 0 {
                    cop[0] += count(i, i, &mut cnt[0]);
                    cop[1] += count(i, i, &mut cnt[1]);
                } else {
                    cop[0] += count(i - j, i - j, &mut cnt[0]);
                    cop[0] += count(i + j, i + j, &mut cnt[0]);
                    cop[1] += count(i - j, i + j, &mut cnt[1]);
                    cop[1] += count(i + j, i - j, &mut cnt[1]);
                }
                op[i - j][i + j] = cop[0].min(cop[1]);
                if i + j + 1 < len {
                    cop[2] += count(i - j, i - j, &mut cnt[2]);
                    cop[2] += count(i + j + 1, i + j + 1, &mut cnt[2]);
                    cop[3] += count(i - j, i + j + 1, &mut cnt[3]);
                    cop[3] += count(i + j + 1, i - j, &mut cnt[3]);
                    op[i - j][i + j + 1] = cop[2].min(cop[3]);
                }
            }
        }
        let mut dp = vec![i32::MAX; len + 1];
        dp[0] = 0;
        for i in 0..len {
            for j in 0..=i {
                dp[i + 1] = dp[i + 1].min(dp[j] + op[j][i]);
            }
        }
        dp[len]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations("abcdf".to_string(), "dacbe".to_string()), 4);
        assert_eq!(
            Solution::min_operations("abceded".to_string(), "baecfef".to_string()),
            4
        );
        assert_eq!(Solution::min_operations("abcdef".to_string(), "fedabc".to_string()), 2);
    }
}
