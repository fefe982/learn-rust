// https://leetcode.com/problems/minimum-seconds-to-equalize-a-circular-array/
// 2808. Minimum Seconds to Equalize a Circular Array
pub struct Solution;
impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let mut pos = std::collections::HashMap::<i32, (usize, usize, usize)>::new();
        let n = nums.len();
        for (i, v) in nums.into_iter().enumerate() {
            if let Some(p) = pos.get_mut(&v) {
                *p = (p.0, i, p.2.max(i - p.1));
            } else {
                pos.insert(v, (i, i, 0));
            }
        }
        let mut min = usize::MAX;
        for (_, (start, end, dist)) in pos {
            min = min.min(dist.max(start + n - end));
        }
        min as i32 / 2
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_seconds() {
        assert_eq!(Solution::minimum_seconds(vec![1, 2, 1, 2]), 1);
        assert_eq!(Solution::minimum_seconds(vec![2, 1, 3, 3, 2]), 2);
        assert_eq!(Solution::minimum_seconds(vec![5, 5, 5, 5]), 0);
    }
}
