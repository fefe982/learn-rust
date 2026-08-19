// https://leetcode.com/problems/distribute-elements-into-two-arrays-i/
// 3069. Distribute Elements Into Two Arrays
pub struct Solution;
impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = vec![nums[0]];
        let mut arr2 = vec![nums[1]];
        for i in 2..nums.len() {
            if arr1.last().unwrap() > arr2.last().unwrap() {
                arr1.push(nums[i]);
            } else {
                arr2.push(nums[i]);
            }
        }
        arr1.append(&mut arr2);
        arr1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn result_array() {
        assert_eq!(Solution::result_array(vec![2, 1, 3]), vec![2, 3, 1]);
        assert_eq!(Solution::result_array(vec![5, 4, 3, 8]), vec![5, 3, 4, 8])
    }
}
