// https://leetcode.com/problems/create-target-array-in-the-given-order/
// 1389. Create Target Array in the Given Order
pub struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut vec = Vec::new();
        for (n, i) in nums.iter().zip(index.iter()) {
            vec.insert(*i as usize, *n);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_target_array() {
        assert_eq!(
            Solution::create_target_array(Vec::from_iter(0..5), vec![0, 1, 2, 2, 1]),
            vec![0, 4, 1, 3, 2]
        );
        assert_eq!(
            Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
            vec![0, 1, 2, 3, 4]
        );
    }
}
