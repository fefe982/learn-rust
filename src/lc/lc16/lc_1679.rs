// https://leetcode.com/problems/max-number-of-k-sum-pairs/
// 1679. Max Number of K-Sum Pairs
pub struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        let mut map = std::collections::HashMap::new();
        for num in nums {
            if let Some(c) = map.get_mut(&(k - num)) {
                if *c > 0 {
                    *c -= 1;
                    cnt += 1;
                    continue;
                }
            }
            map.entry(num).and_modify(|c| *c += 1).or_insert(1);
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_operations() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }
}
