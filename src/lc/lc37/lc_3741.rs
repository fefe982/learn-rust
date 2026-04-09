// https://leetcode.com/problems/minimum-distance-between-three-equal-elements-ii/
// 3741. Minimum Distance Between Three Equal Elements II
pub struct Solution;
impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        let mut map = std::collections::HashMap::<i32, Vec<usize>>::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(vec) = map.get_mut(&num) {
                vec.push(i);
                if vec.len() >= 3 {
                    let n = vec.len();
                    ans = ans.min((vec[n - 1] - vec[n - 3]) as i32);
                }
            } else {
                map.insert(num, vec![i]);
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans * 2
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_distance() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
        assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
        assert_eq!(Solution::minimum_distance(vec![1]), -1);
    }
}
