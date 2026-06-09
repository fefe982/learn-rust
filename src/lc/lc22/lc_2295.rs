// https://leetcode.com/problems/replace-elements-in-an-array/
// 2295. Replace Elements in an Array
pub struct Solution;
impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, &n) in nums.iter().enumerate() {
            map.insert(n, i);
        }
        for op in operations {
            if let [old, new] = op.as_slice() {
                let v = map.remove(old).unwrap();
                map.insert(*new, v);
            }
        }
        let mut nums = nums;
        for (n, i) in map {
            nums[i] = n;
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_array_change() {
        assert_eq!(
            Solution::array_change(vec![1, 2, 4, 6], vec_vec![[1, 3], [4, 7], [6, 1]]),
            vec![3, 2, 7, 1]
        );
        assert_eq!(
            Solution::array_change(vec![1, 2], vec_vec![[1, 3], [2, 1], [3, 2]]),
            vec![2, 1]
        );
    }
}
