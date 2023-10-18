// https://leetcode.com/problems/maximal-score-after-applying-k-operations/
// 2530. Maximal Score After Applying K Operations
pub struct Solution;
impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut q: std::collections::BinaryHeap<i32> = nums.into_iter().collect();
        let mut ans = 0;
        for _ in 0..k {
            let cur = q.pop().unwrap();
            ans += cur as i64;
            q.push((cur + 2) / 3);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_kelements() {
        assert_eq!(Solution::max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
        assert_eq!(Solution::max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }
}
