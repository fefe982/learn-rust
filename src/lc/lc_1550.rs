// https://leetcode.com/problems/three-consecutive-odds/
// 1550. Three Consecutive Odds
pub struct Solution;
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut cnt = 0;
        for n in arr {
            if n % 2 == 1 {
                cnt += 1;
            } else {
                cnt = 0;
            }
            if cnt >= 3 {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_three_consecutive_odds() {
        assert_eq!(Solution::three_consecutive_odds(vec![2, 6, 4, 1]), false);
        assert_eq!(
            Solution::three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true
        );
    }
}
