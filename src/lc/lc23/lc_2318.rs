// https://leetcode.com/problems/number-of-distinct-roll-sequences/
// 2318. Number of Distinct Roll Sequences
pub struct Solution;
impl Solution {
    pub fn distinct_sequences(n: i32) -> i32 {
        if n == 1 {
            return 6;
        }
        let prime = vec![
            vec![0, 1, 1, 1, 1, 1],
            vec![1, 0, 1, 0, 1, 0],
            vec![1, 1, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 0],
            vec![1, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 1, 0],
        ];
        let mut cnt = prime.clone();
        for _ in 2..n {
            let mut ncnt = vec![vec![0; 6]; 6];
            for i in 0..6 {
                let mut sum = 0;
                for k in 0..6 {
                    sum += cnt[k][i];
                }
                for j in 0..6 {
                    if prime[i][j] == 0 {
                        continue;
                    }
                    ncnt[i][j] = sum - cnt[j][i];
                    ncnt[i][j] %= 1000000007;
                }
            }
            cnt = ncnt;
        }
        (cnt.into_iter().map(|v| v.into_iter().sum::<i64>()).sum::<i64>() % 1000000007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distinct_sequences() {
        assert_eq!(Solution::distinct_sequences(4), 184);
        assert_eq!(Solution::distinct_sequences(2), 22);
    }
}
