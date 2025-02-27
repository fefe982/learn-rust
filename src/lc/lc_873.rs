// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
// 873. Length of Longest Fibonacci Subsequence
pub struct Solution;
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut set = std::collections::HashMap::new();
        for (i, &num) in arr.iter().enumerate() {
            set.insert(num, i);
        }
        let mut max_len = 0;
        let mut dp = vec![vec![2; arr.len()]; arr.len()];
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                let a = arr[i];
                let b = arr[j];
                if let Some(&k) = set.get(&(b - a)) {
                    if k >= i {
                        break;
                    }
                    dp[i][j] = dp[k][i] + 1;
                    max_len = max_len.max(dp[i][j]);
                }
            }
        }
        max_len
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn len_longest_fib_subseq() {
        assert_eq!(Solution::len_longest_fib_subseq(vec![1, 2, 3, 4, 5, 6, 7, 8]), 5);
        assert_eq!(Solution::len_longest_fib_subseq(vec![1, 3, 7, 11, 12, 14, 18]), 3);
    }
}
