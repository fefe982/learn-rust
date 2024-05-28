// https://leetcode.com/problems/maximum-number-of-ways-to-partition-an-array/
// 2025. Maximum Number of Ways to Partition an Array
pub struct Solution;
impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre = vec![0; nums.len()];
        let mut post = vec![0; nums.len()];
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num as i64;
            pre[i] = sum;
        }
        sum = 0;
        for (i, &num) in nums.iter().enumerate().rev() {
            sum += num as i64;
            post[i] = sum;
        }
        let mut left = std::collections::HashMap::new();
        let mut right = std::collections::HashMap::new();
        for i in 0..nums.len() - 1 {
            let diff = pre[i] - post[i + 1];
            *right.entry(diff).or_default() += 1;
        }
        let mut ans = *right.get(&0).unwrap_or(&0);
        for i in 0..nums.len() {
            let d = (k - nums[i]) as i64;
            let cnt = left.get(&d).copied().unwrap_or(0) + right.get(&-d).copied().unwrap_or(0);
            ans = ans.max(cnt);
            if i == nums.len() - 1 {
                break;
            }
            let diff = pre[i] - post[i + 1];
            *left.entry(diff).or_default() += 1;
            *right.entry(diff).or_default() -= 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ways_to_partition() {
        assert_eq!(Solution::ways_to_partition(vec![2, -1, 2], 3), 1);
        assert_eq!(Solution::ways_to_partition(vec![0, 0, 0], 1), 2);
        assert_eq!(
            Solution::ways_to_partition(vec![22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14], -33),
            4
        );
    }
}
