// https://leetcode.com/problems/132-pattern/
// 456. 132 Pattern
pub struct Solution;
impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut third = i32::MIN;
        for num in nums.into_iter().rev() {
            if num < third {
                return true;
            }
            while let Some(&t) = stack.last() {
                if t < num {
                    stack.pop();
                    third = t;
                } else {
                    break;
                }
            }
            stack.push(num);
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find132pattern() {
        assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
        assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
    }
}
