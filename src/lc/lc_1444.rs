// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/
// 1444. Number of Ways of Cutting a Pizza
pub struct Solution;
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let modulo = 1000000007;
        let mut sum = vec![vec![0; pizza[0].len() + 1]; pizza.len() + 1];
        let mut dp = vec![vec![vec![0; k as usize]; pizza[0].len() + 1]; pizza.len() + 1];
        for i in (0..pizza.len()).rev() {
            for j in (0..pizza[i].len()).rev() {
                let c = pizza[i].as_bytes()[j];
                sum[i][j] = sum[i + 1][j] + sum[i][j + 1] - sum[i + 1][j + 1];
                if c != b'.' {
                    sum[i][j] += 1;
                }
            }
        }
        for i in (0..pizza.len()).rev() {
            for j in (0..pizza[i].len()).rev() {
                if sum[i][j] != 0 {
                    dp[i][j][0] = 1;
                }
                for l in 1..k as usize {
                    for q in j + 1..pizza[i].len() {
                        if sum[i][j] > sum[i][q] {
                            dp[i][j][l] = (dp[i][j][l] + dp[i][q][l - 1]) % modulo;
                        }
                    }
                    for q in i + 1..pizza.len() {
                        if sum[i][j] > sum[q][j] {
                            dp[i][j][l] = (dp[i][j][l] + dp[q][j][l - 1]) % modulo;
                        }
                    }
                }
            }
        }
        dp[0][0][k as usize - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_reorder() {
        assert_eq!(
            Solution::ways(
                vec![
                    String::from("A.."),
                    String::from("AAA"),
                    String::from("...")
                ],
                3
            ),
            3
        );
        assert_eq!(
            Solution::ways(
                vec![
                    String::from("A.."),
                    String::from("AA."),
                    String::from("...")
                ],
                3
            ),
            1
        );
        assert_eq!(
            Solution::ways(
                vec![
                    String::from("A.."),
                    String::from("A.."),
                    String::from("...")
                ],
                1
            ),
            1
        );
    }
}
