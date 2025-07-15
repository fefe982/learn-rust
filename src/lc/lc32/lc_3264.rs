// https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/
// 3264. Final Array State After K Multiplication Operations I
pub struct Solution;
impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut h = std::collections::BinaryHeap::new();
        let mut max = 1;
        for (i, &n) in nums.iter().enumerate() {
            h.push(std::cmp::Reverse((n, i)));
            max = max.max(n);
        }
        let mut k = k;
        let mut nums = nums;
        while k > 0 {
            let std::cmp::Reverse(mut top) = h.pop().unwrap();
            top.0 *= multiplier;
            h.push(std::cmp::Reverse(top));
            nums[top.1] = top.0;
            k -= 1;
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_final_state() {
        assert_eq!(
            Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2),
            vec![8, 4, 6, 5, 6]
        );
        assert_eq!(Solution::get_final_state(vec![1, 2], 3, 4), [16, 8]);
    }
}
