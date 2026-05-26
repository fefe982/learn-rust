// https://leetcode.com/problems/maximize-the-topmost-element-after-k-moves/
// 2202. Maximize the Topmost Element After K Moves
pub struct Solution;
impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            nums[0]
        } else if nums.len() == 1 {
            if k % 2 == 0 {
                nums[0]
            } else {
                -1
            }
        } else if k as usize > nums.len() {
            nums.into_iter().max().unwrap()
        } else {
            let mut ans = -1;
            for i in 0..k as usize - 1 {
                ans = ans.max(nums[i]);
            }
            if (k as usize) < nums.len() {
                ans = ans.max(nums[k as usize]);
            }
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_top() {
        assert_eq!(Solution::maximum_top(vec![5, 2, 2, 4, 0, 6], 4), 5);
        assert_eq!(Solution::maximum_top(vec![2], 1), -1);
        assert_eq!(Solution::maximum_top(vec![1, 2, 3], 2), 3);
    }
}
