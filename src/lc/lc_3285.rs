// https://leetcode.com/problems/find-indices-of-stable-mountains/
// 3285. Find Indices of Stable Mountains
pub struct Solution;
impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..height.len() {
            if height[i - 1] > threshold {
                res.push(i as i32);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stable_mountains() {
        assert_eq!(Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2), [3, 4]);
        assert_eq!(Solution::stable_mountains(vec![10, 1, 10, 1, 10], 3), [1, 3]);
        assert_eq!(Solution::stable_mountains(vec![10, 1, 10, 1, 10], 10), []);
    }
}
