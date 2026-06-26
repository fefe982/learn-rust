// https://leetcode.com/problems/destroy-sequential-targets/
// 2453. Destroy Sequential Targets
pub struct Solution;
impl Solution {
    pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut ans = std::i32::MAX;
        let mut max_cnt = 0;
        for &num in &nums {
            *cnt.entry(num % space).or_insert(0) += 1;
        }
        for num in nums {
            let c = cnt[&(num % space)];
            if c > max_cnt || (c == max_cnt && num < ans) {
                max_cnt = c;
                ans = num;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn destroy_targets() {
        assert_eq!(Solution::destroy_targets(vec![3, 7, 8, 1, 1, 5], 2), 1);
        assert_eq!(Solution::destroy_targets(vec![1, 3, 5, 2, 4, 6], 2), 1);
        assert_eq!(Solution::destroy_targets(vec![6, 2, 5], 100), 2);
    }
}
