// https://leetcode.com/problems/sort-array-by-parity-ii/
// 922. Sort Array By Parity II
pub struct Solution;
impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut i = 0;
        let mut j = 1;
        while i < nums.len() && j < nums.len() {
            while i < nums.len() && nums[i] % 2 == 0 {
                i += 2;
            }
            while j < nums.len() && nums[j] % 2 == 1 {
                j += 2;
            }
            if i < nums.len() && j < nums.len() {
                nums.swap(i, j);
            }
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums: Vec<i32>) {
        let ans = Solution::sort_array_by_parity_ii(nums.clone());
        let mut cnt = std::collections::HashMap::new();
        for n in nums {
            *cnt.entry(n).or_insert(0) += 1;
        }
        for (i, n) in ans.into_iter().enumerate() {
            assert!(n % 2 == i as i32 % 2);
            *cnt.entry(n).or_insert(0) -= 1;
        }
        for (_, v) in cnt {
            assert!(v == 0);
        }
    }
    #[test]
    fn sort_array_by_parity_ii() {
        check(vec![4, 2, 5, 7]);
        check(vec![2, 3]);
    }
}
