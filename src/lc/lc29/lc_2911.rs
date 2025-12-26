// https://leetcode.com/problems/minimum-changes-to-make-k-semi-palindromes/
// 2911. Minimum Changes to Make K Semi-Palindromes
pub struct Solution;
impl Solution {
    fn get_fac(n: usize) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; n + 1];
        for i in 1..=n / 2 {
            for j in 2..=n / i {
                res[i * j].push(i);
            }
        }
        res
    }
    fn count(s: &[u8], fac: &Vec<Vec<usize>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; s.len() + 1]; s.len()];
        for i in 0..s.len() {
            for j in i + 2..=s.len() {
                let ss = &s[i..j];
                let mut min = i32::MAX;
                for &f in &fac[ss.len()] {
                    let l = ss.len() / f;
                    let mut ans = 0;
                    for g in 0..f {
                        for j in 0..l / 2 {
                            if ss[j * f + g] != ss[(l - j - 1) * f + g] {
                                ans += 1;
                            }
                        }
                    }
                    min = min.min(ans);
                }
                res[i][j] = min;
            }
        }
        res
    }
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let fac = Self::get_fac(s.len());
        let chng = Self::count(s, &fac);
        let k = k as usize;
        let mut dp = vec![vec![s.len() as i32; k + 1]; s.len() + 1];
        dp[0][0] = 0;
        for len in 1..=s.len() {
            for i in 0..len - 1 {
                for j in 1..=k {
                    dp[len][j] = dp[len][j].min(dp[i][j - 1] + chng[i][len]);
                }
            }
        }
        dp[s.len()][k]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_changes() {
        assert_eq!(Solution::minimum_changes("acba".to_string(), 2), 2);
        assert_eq!(Solution::minimum_changes("abcac".to_string(), 2), 1);
        assert_eq!(Solution::minimum_changes("abcdef".to_string(), 2), 2);
        assert_eq!(Solution::minimum_changes("aabbaa".to_string(), 3), 0);
    }
}
