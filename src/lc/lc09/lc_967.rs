// https://leetcode.com/problems/numbers-with-same-consecutive-differences
// 967. Numbers With Same Consecutive Differences
pub struct Solution;
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];
        fn dfs(n: i32, k: i32, num: i32, digit: i32, res: &mut Vec<i32>) {
            if n == 0 {
                res.push(num);
                return;
            }
            if digit - k >= 0 {
                dfs(n - 1, k, num * 10 + digit - k, digit - k, res);
            }
            if k != 0 && digit + k <= 9 {
                dfs(n - 1, k, num * 10 + digit + k, digit + k, res);
            }
        }
        for i in 1..10 {
            dfs(n - 1, k, i, i, &mut res);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn nums_same_consec_diff() {
        assert_sort_eq!(
            Solution::nums_same_consec_diff(2, 0),
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99]
        );
        assert_sort_eq!(Solution::nums_same_consec_diff(3, 7), vec![181, 292, 707, 818, 929]);
        assert_sort_eq!(
            Solution::nums_same_consec_diff(2, 1),
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
        );
    }
}
