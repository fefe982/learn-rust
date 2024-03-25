// https://leetcode.com/problems/find-all-duplicates-in-an-array/
// 442. Find All Duplicates in an Array
pub struct Solution;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ans = vec![];
        let mut nlast = 0;
        for i in 0..nums.len() {
            let mut n = nums[i] as usize;
            if n == 0 {
                continue;
            }
            nums[i] = -1;
            loop {
                if n >= nums.len() {
                    nlast += 1;
                    if nlast == 2 {
                        ans.push(n as i32);
                    }
                    break;
                }
                if nums[n] == -1 {
                    nums[n] = 0;
                    break;
                }
                if nums[n] == 0 {
                    ans.push(n as i32);
                    break;
                }
                let next = nums[n] as usize;
                nums[n] = 0;
                n = next;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_duplicates() {
        assert_eq!(Solution::find_duplicates(vec![2, 2]), vec![2]);
        assert_eq!(Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
        assert_eq!(Solution::find_duplicates(vec![1]), vec![]);
    }
}
