// https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/
// 3289. The Two Sneaky Numbers of Digitville
pub struct Solution;
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut res = vec![];
        for i in 0..nums.len() {
            let mut n = nums[i] as usize;
            if n == i {
                continue;
            }
            nums[i] = -1;
            loop {
                if nums[n] == n as i32 {
                    res.push(n as i32);
                    break;
                }
                if nums[n] == -1 {
                    nums[n] = n as i32;
                    break;
                }
                let next = nums[n] as usize;
                nums[n] = n as i32;
                n = next;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn get_sneaky_numbers() {
        assert_sort_eq!(Solution::get_sneaky_numbers(vec![0, 1, 1, 0]), vec![0, 1]);
        assert_sort_eq!(Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]), vec![2, 3]);
        assert_sort_eq!(
            Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]),
            vec![4, 5]
        );
    }
}
