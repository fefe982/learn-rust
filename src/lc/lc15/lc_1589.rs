// https://leetcode.com/problems/maximum-sum-obtained-of-any-permutation/
// 1589. Maximum Sum Obtained of Any Permutation
pub struct Solution;
impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; nums.len() + 1];
        for req in requests {
            cnt[req[0] as usize] += 1;
            cnt[req[1] as usize + 1] -= 1;
        }
        for i in 1..cnt.len() {
            cnt[i] += cnt[i - 1];
        }
        cnt.pop();
        cnt.sort();
        let mut nums = nums;
        nums.sort();
        (nums
            .iter()
            .zip(cnt.iter())
            .map(|(&x, &y)| x as i64 * y as i64)
            .sum::<i64>()
            % 1_000_000_007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_sum_range_query() {
        assert_eq!(
            Solution::max_sum_range_query(vec![1, 2, 3, 4, 5], vec_vec![[1, 3], [0, 1]]),
            19
        );
        assert_eq!(
            Solution::max_sum_range_query(vec![1, 2, 3, 4, 5, 6], vec_vec![[0, 1]]),
            11
        );
        assert_eq!(
            Solution::max_sum_range_query(vec![1, 2, 3, 4, 5, 10], vec_vec![[0, 2], [1, 3], [1, 1]]),
            47
        );
    }
}
