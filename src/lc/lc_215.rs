// https://leetcode.com/problems/kth-largest-element-in-an-array/
// 215. Kth Largest Element in an Array
pub struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize + 1;
        let mut q = std::collections::BinaryHeap::new();
        for i in 0..nums.len() {
            if i >= k {
                let &t = q.peek().unwrap();
                if nums[i] < t {
                    q.pop();
                }
            }
            if q.len() < k {
                q.push(nums[i]);
            }
        }
        *q.peek().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
