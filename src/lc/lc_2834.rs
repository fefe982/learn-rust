// https://leetcode.cn/problems/find-the-minimum-possible-sum-of-a-beautiful-array/
// 2834. Find the Minimum Possible Sum of a Beautiful Array
pub struct Solution;
impl Solution {
    pub fn minimum_possible_sum(n: i32, target: i32) -> i32 {
        let n = n as i64;
        let target = target as i64;
        let mut sum = n * (n + 1) / 2;
        if n > target / 2 {
            sum += (target - target / 2 - 1) * (n - target / 2);
        }
        (sum % 1_0000_0000_7) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::minimum_possible_sum(2, 3), 4);
        assert_eq!(Solution::minimum_possible_sum(3, 3), 8);
        assert_eq!(Solution::minimum_possible_sum(1, 1), 1);
    }
}
