// https://leetcode.com/problems/maximize-score-after-n-operations/
// 1799. Maximize Score After N Operations
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
            if b == 0 {
                return a;
            }
        }
    }
    fn max_score_helper(
        idx: i32,
        nums: &Vec<i32>,
        used: i32,
        cache: &mut std::collections::HashMap<i32, i32>,
    ) -> i32 {
        if idx as usize * 2 > nums.len() {
            return 0;
        }
        if let Some(&r) = cache.get(&used) {
            return r;
        }
        let mut max_v = 0;
        for i in 0..(nums.len() - 1) {
            for j in i + 1..nums.len() {
                let m = 1 << i | 1 << j;
                if used & m == 0 {
                    max_v = std::cmp::max(
                        max_v,
                        idx * Self::gcd(nums[i], nums[j])
                            + Self::max_score_helper(idx + 1, nums, used | m, cache),
                    )
                }
            }
        }
        cache.insert(used, max_v);
        max_v
    }
    pub fn max_score(nums: Vec<i32>) -> i32 {
        Self::max_score_helper(1, &nums, 0, &mut std::collections::HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(vec![1, 2]), 1);
        assert_eq!(Solution::max_score(vec![3, 4, 6, 8]), 11);
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6]), 14);
    }
}
