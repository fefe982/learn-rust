// https://leetcode.com/problems/sum-of-even-numbers-after-queries/
// 985. Sum of Even Numbers After Queries
pub struct Solution;
impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = nums.iter().filter(|&x| x % 2 == 0).sum();
        let mut nums = nums;
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let (val, index) = (q[0], q[1] as usize);
            if nums[index] % 2 == 0 {
                sum -= nums[index];
            }
            nums[index] += val;
            if nums[index] % 2 == 0 {
                sum += nums[index];
            }
            ans.push(sum);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn sum_even_after_queries() {
        assert_eq!(
            Solution::sum_even_after_queries(vec![1, 2, 3, 4], vec_vec![[1, 0], [-3, 1], [-4, 0], [2, 3]]),
            [8, 6, 2, 4]
        );
        assert_eq!(Solution::sum_even_after_queries(vec![1], vec_vec![[4, 0]]), [0]);
    }
}
