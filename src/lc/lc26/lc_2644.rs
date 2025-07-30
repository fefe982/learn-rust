// https://leetcode.com/problems/find-the-maximum-divisibility-score/
// 2644. Find the Maximum Divisibility Score
pub struct Solution;
impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut min_divisor = i32::MAX;
        let mut max_cnt = 0;
        for divisor in divisors {
            let cnt = nums.iter().filter(|&&num| num % divisor == 0).count();
            if cnt > max_cnt {
                max_cnt = cnt;
                min_divisor = divisor;
            } else if cnt == max_cnt && divisor < min_divisor {
                min_divisor = divisor;
            }
        }
        min_divisor
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_div_score() {
        assert_eq!(Solution::max_div_score(vec![4, 7, 9, 3, 9], vec![5, 2, 3]), 3);
        assert_eq!(Solution::max_div_score(vec![20, 14, 21, 10], vec![5, 7, 5]), 5);
        assert_eq!(Solution::max_div_score(vec![12], vec![10, 16]), 10);
    }
}
