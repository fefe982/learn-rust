// https://leetcode.com/problems/sorting-three-groups/
// 2826. Sort the Students by Their Kth Score
pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut q = Vec::with_capacity(nums.len());
        for &n in &nums {
            let i = q.partition_point(|&x| x <= n);
            if i < q.len() {
                q[i] = n;
            } else {
                q.push(n);
            }
        }
        (nums.len() - q.len()) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![2, 1, 3, 2, 1]), 3);
        assert_eq!(Solution::minimum_operations(vec![1, 3, 2, 1, 3, 3]), 2);
        assert_eq!(Solution::minimum_operations(vec![2, 2, 2, 2, 3, 3]), 0);
    }
}
