// https://leetcode.com/problems/longest-subsequence-with-limited-sum/
// 2389. Longest Subsequence With Limited Sum
pub struct Solution;
impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut last = nums[0];
        for n in nums[1..].iter_mut() {
            *n += last;
            last = *n;
        }
        queries
            .iter()
            .map(|q| match nums.binary_search(&q) {
                Ok(idx) => idx + 1,
                Err(idx) => idx,
            } as i32)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn answer_queries() {
        assert_eq!(
            Solution::answer_queries(
                vec![736411, 184882, 914641, 37925, 214915],
                vec![718089, 665450]
            ),
            vec![3, 3]
        );
        assert_eq!(
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }
}
