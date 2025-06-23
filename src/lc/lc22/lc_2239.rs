// https://leetcode.com/problems/find-closest-number-to-zero/
// 2239. Find Closest Number to Zero
pub struct Solution;
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(i32::MAX, |acc, x| {
            if acc.abs() > x.abs() || (acc.abs() == x.abs() && x > acc) {
                x
            } else {
                acc
            }
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_closest_number() {
        assert_eq!(Solution::find_closest_number(vec![-4, -2, 1, 4, 8]), 1);
        assert_eq!(Solution::find_closest_number(vec![2, -1, 1]), 1);
    }
}
