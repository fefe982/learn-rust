// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/
// 3191. Minimum Operations to Make Binary Array Elements Equal to One I
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut q = std::collections::VecDeque::<usize>::new();
        let mut res = 0;
        let len = nums.len();
        for (i, n) in nums.into_iter().enumerate() {
            while let Some(j) = q.front() {
                if j + 2 < i {
                    q.pop_front();
                } else {
                    break;
                }
            }
            if n % 2 == q.len() as i32 % 2 {
                if i + 2 >= len {
                    return -1;
                }
                q.push_back(i);
                res += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
        assert_eq!(Solution::min_operations(vec![0, 1, 1, 1]), -1);
    }
}
