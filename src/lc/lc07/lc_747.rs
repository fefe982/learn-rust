// https://leetcode.com/problems/largest-number-at-least-twice-of-others/
// 747. Largest Number At Least Twice of Others
pub struct Solution;
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut l = [0; 2];
        let mut il = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if n >= l[1] {
                l[1] = n;
            }
            if n >= l[0] {
                l[1] = l[0];
                l[0] = n;
                il = i
            }
        }
        if l[0] >= l[1] * 2 {
            il as i32
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dominator_index() {
        assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
        assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
    }
}
