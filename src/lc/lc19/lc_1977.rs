// https://leetcode.com/problems/number-of-ways-to-separate-numbers/
// 1977. Number of Ways to Separate Numbers
pub struct Solution;
impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let num = num.as_bytes();
        let m = 1000000007;
        if num[0] == b'0' {
            return 0;
        }
        let mut common_length = vec![vec![0; num.len()]; num.len()];
        for i in (0..num.len()).rev() {
            if i == num.len() - 1 {
                common_length[i][i] = 1;
                continue;
            }
            for j in (i..num.len()).rev() {
                if j == num.len() - 1 {
                    common_length[i][j] = if num[i] == num[j] { 1 } else { 0 };
                } else {
                    common_length[i][j] = if num[i] == num[j] {
                        common_length[i + 1][j + 1] + 1
                    } else {
                        0
                    }
                }
            }
        }
        let mut dp = vec![vec![0i64; num.len() + 1]; num.len() + 2];
        dp[1][0] = 1;
        for i in 0..=num.len() {
            if i > 0 {
                dp[i + 1][i] = dp[i][i];
            }
            for j in i + 1..=num.len() {
                dp[i + 1][j] = dp[i][j];
                if num[i] == b'0' {
                    continue;
                }
                let len = j - i;
                let low = if len > i {
                    0
                } else {
                    let c = common_length[i - len][i];
                    if c >= len || num[i - len + c] < num[i + c] {
                        i - len
                    } else {
                        i - len + 1
                    }
                };
                dp[i + 1][j] = (dp[i + 1][j] + dp[i + 1][i] - dp[low][i] + m) % m;
            }
        }
        dp[num.len()][num.len()] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_of_combinations() {
        assert_eq!(Solution::number_of_combinations("1203".to_owned()), 2);
        assert_eq!(Solution::number_of_combinations("999".to_owned()), 3);
        assert_eq!(Solution::number_of_combinations("9999999999999".to_owned()), 101);
        assert_eq!(Solution::number_of_combinations("327".to_owned()), 2);
        assert_eq!(Solution::number_of_combinations("094".to_owned()), 0);
        assert_eq!(Solution::number_of_combinations("0".to_owned()), 0);
    }
}
