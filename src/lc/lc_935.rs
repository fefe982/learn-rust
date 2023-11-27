// https://leetcode.com/problems/knight-dialer/
// 935. Knight Dialer
pub struct Solution;
impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let jump = vec![
            vec![4, 6],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![0, 3, 9],
            vec![],
            vec![0, 1, 7],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4],
        ];
        let mut dp = vec![1i64; 10];
        let m = 1_000_000_007;
        for _ in 1..n {
            let mut ndp = vec![0; 10];
            for j in 0..10 {
                for &k in &jump[j] {
                    ndp[k] = (ndp[k] + dp[j]) % m;
                }
            }
            dp = ndp;
        }
        (dp.iter().sum::<i64>() % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_knight_dialer() {
        assert_eq!(Solution::knight_dialer(1), 10);
        assert_eq!(Solution::knight_dialer(2), 20);
        assert_eq!(Solution::knight_dialer(3131), 136006598);
    }
}
