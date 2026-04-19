// https://leetcode.com/problems/two-furthest-houses-with-different-colors/
// 2078. Two Furthest Houses With Different Colors
pub struct Solution;
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let n = colors.len();
        if colors[0] != colors[n - 1] {
            return (n - 1) as i32;
        }
        let mut left = 0;
        while left < n && colors[left] == colors[0] {
            left += 1;
        }
        let mut right = n;
        while right > 0 && colors[right - 1] == colors[0] {
            right -= 1;
        }
        (right as i32 - 1).max((n - 1 - left) as i32)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_distance() {
        assert_eq!(Solution::max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
        assert_eq!(Solution::max_distance(vec![1, 8, 3, 8, 3]), 4);
        assert_eq!(Solution::max_distance(vec![0, 1]), 1);
    }
}
