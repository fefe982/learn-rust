// https://leetcode.com/problems/trapping-rain-water/
// 42. Trapping Rain Water
pub struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut max_l = vec![0];
        for idx in 0..height.len() - 1 {
            max_l.push(std::cmp::max(max_l[idx], height[idx]))
        }
        let mut sum = 0;
        let mut max_r = height[height.len() - 1];
        for idx in (1..height.len() - 1).rev() {
            sum += std::cmp::max(std::cmp::min(max_l[idx], max_r) - height[idx], 0);
            max_r = std::cmp::max(height[idx], max_r);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
