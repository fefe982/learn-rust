// https://leetcode.com/problems/contains-duplicate/
// 217. Contains Duplicate
pub struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::with_capacity(nums.len());
        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
