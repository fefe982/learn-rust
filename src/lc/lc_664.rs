// https://leetcode.com/problems/strange-printer/description/
// 664. Strange Printer
pub struct Solution;
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let mut v = vec![];
        let mut last = 0 as char;
        for c in s.chars() {
            if c != last {
                v.push(c);
            }
            last = c;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![1]; v.len()];
        for (i, dpi) in dp.iter_mut().enumerate() {
            dpi.resize(v.len() - i, 0);
        }
        for l in 1..v.len() {
            for i in 0..v.len() - l {
                dp[i][l] = dp[i][l - 1] + 1;
                for j in 0..l {
                    if v[i + l] == v[i + j] {
                        dp[i][l] = dp[i][l].min(dp[i][j] + dp[i + j + 1][l - j - 2])
                    }
                }
            }
        }
        dp[0][v.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strange_printer() {
        // assert_eq!(Solution::strange_printer(String::from("aaabbb")), 2);
        assert_eq!(Solution::strange_printer(String::from("aba")), 2);
    }
}
