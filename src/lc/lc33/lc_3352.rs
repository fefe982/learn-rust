// https://leetcode.com/problems/count-k-reducible-numbers-less-than-n/
// 3352. Count K-Reducible Numbers Less Than N
pub struct Solution;
impl Solution {
    pub fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
        let m = 1_000_000_007;
        let s = s.as_bytes();
        let mut ks = vec![0; s.len()];
        for i in 2..s.len() {
            ks[i] = ks[i.count_ones() as usize] + 1;
        }
        let mut ione = Vec::with_capacity(s.len());
        for i in 0..s.len() {
            if s[i] == b'1' {
                ione.push(i);
            }
        }
        let mut c = vec![vec![1; s.len()]; s.len()];
        for i in 2..s.len() {
            for j in 1..i {
                c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % m;
            }
        }
        let mut ans = m - 1;
        for i in 0..ione.len() {
            for j in 0..s.len() - ione[i] {
                if ks[i + j] < k {
                    ans = (ans + c[s.len() - ione[i] - 1][j]) % m
                }
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_k_reducible_numbers() {
        assert_eq!(Solution::count_k_reducible_numbers("111".to_string(), 1), 3);
        assert_eq!(Solution::count_k_reducible_numbers("1000".to_string(), 2), 6);
        assert_eq!(Solution::count_k_reducible_numbers("1".to_string(), 3), 0);
    }
}
