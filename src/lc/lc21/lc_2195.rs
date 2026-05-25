// https://leetcode.com/problems/append-k-integers-with-minimal-sum/
// 2195. Append K Integers With Minimal Sum
pub struct Solution;
impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        let mut prev = 0;
        let mut k = k as i64;
        for num in nums {
            if num as i64 > prev + 1 {
                let end = (num as i64 - 1).min(prev + k);
                ans += (prev + 1 + end) * (end - prev) / 2;
                k -= end - prev;
            }
            if k == 0 {
                break;
            }
            prev = num as i64;
        }
        if k > 0 {
            ans += (prev + 1 + prev + k) * k / 2;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimal_k_sum() {
        assert_eq!(Solution::minimal_k_sum(vec![1, 4, 25, 10, 25], 2), 5);
        assert_eq!(Solution::minimal_k_sum(vec![5, 6], 6), 25);
    }
}
