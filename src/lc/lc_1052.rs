// https://leetcode.com/problems/grumpy-bookstore-owner/
// 1052. Grumpy Bookstore Owner
pub struct Solution;
impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut satisfied = 0;
        let mut max_addition = 0;
        let mut addition = 0;
        for i in 0..customers.len() {
            if grumpy[i] == 0 {
                satisfied += customers[i];
            } else {
                addition += customers[i];
            }
            if i >= minutes as usize && grumpy[i - minutes as usize] == 1 {
                addition -= customers[i - minutes as usize];
            }
            max_addition = max_addition.max(addition);
        }
        satisfied + max_addition
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_satisfied() {
        assert_eq!(
            Solution::max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 1], 3),
            16
        );
        assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    }
}
