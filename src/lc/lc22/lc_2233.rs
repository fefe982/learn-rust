// https://leetcode.com/problems/maximum-product-after-k-increments/
// 2233. Maximum Product After K Increments
pub struct Solution;
impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut k = k;
        let mut cnt2 = 0;
        let mut idxn = 0;
        let mut inc = 0;
        for i in 0..nums.len() {
            let next = if i == nums.len() - 1 { i32::MAX } else { nums[i + 1] };
            let diff = next - nums[i];
            if diff as i64 * (i as i64 + 1) <= k as i64 {
                idxn = i + 1;
                k -= diff * (i as i32 + 1);
            } else {
                inc = k / (i as i32 + 1);
                idxn = i;
                cnt2 = k % (i as i32 + 1);
                break;
            }
        }
        let mut ans = 1;
        for _ in 0..cnt2 {
            ans = ans * (nums[idxn] + inc + 1) as i64 % 1000000007;
        }
        for _ in cnt2..idxn as i32 + 1 {
            ans = ans * (nums[idxn] + inc) as i64 % 1000000007;
        }
        for i in idxn as usize + 1..nums.len() {
            ans = ans * nums[i] as i64 % 1000000007;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_product() {
        assert_eq!(Solution::maximum_product(vec![21, 100], 58), 7900);
        assert_eq!(Solution::maximum_product(vec![0, 4], 5), 20);
        assert_eq!(Solution::maximum_product(vec![6, 3, 3, 2], 2), 216);
    }
}
