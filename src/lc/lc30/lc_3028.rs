// https://leetcode.com/problems/ant-on-the-boundary/
// 3028. Ant on the Boundary
pub struct Solution;
impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut cnt = 0;
        for n in nums {
            pos += n;
            if pos == 0 {
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn return_to_boundary_count() {
        assert_eq!(Solution::return_to_boundary_count(vec![2, 3, -5]), 1);
        assert_eq!(Solution::return_to_boundary_count(vec![3, 2, -3, -4]), 0);
    }
}
