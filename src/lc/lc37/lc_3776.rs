// https://leetcode.com/problems/minimum-moves-to-balance-circular-array/
// 3776. Minimum Moves to Balance Circular Array
pub struct Solution;
impl Solution {
    pub fn min_moves(balance: Vec<i32>) -> i64 {
        let mut j = balance.len();
        let mut sum = 0;
        for i in 0..balance.len() {
            sum += balance[i] as i64;
            if balance[i] < 0 {
                j = i;
            }
        }
        if j == balance.len() {
            return 0;
        }
        if sum < 0 {
            return -1;
        }
        sum = 0;
        let l = balance.len();
        let mut s = -balance[j] as i64;
        for i in 1..balance.len() {
            let n = balance[(j + i) % l] as i64 + balance[(j + l - i) % l] as i64;
            sum += i as i64 * s.min(n);
            s -= s.min(n);
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_moves() {
        assert_eq!(Solution::min_moves(vec![5, 1, -4]), 4);
        assert_eq!(Solution::min_moves(vec![1, 2, -5, 2]), 6);
        assert_eq!(Solution::min_moves(vec![-3, 2]), -1);
    }
}
