// https://leetcode.com/problems/subsequence-sum-after-capping-elements/
// 3685. Subsequence Sum After Capping Elements
pub struct Solution;
impl Solution {
    pub fn subsequence_sum_after_capping(nums: Vec<i32>, k: i32) -> Vec<bool> {
        let mut sum = vec![false; k as usize + 1];
        sum[0] = true;
        let mut ans = Vec::with_capacity(nums.len());
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        for x in 1..=nums.len() {
            if sum[k as usize] {
                ans.push(true);
                continue;
            }
            while i < nums.len() && nums[i] <= x as i32 {
                for s in (nums[i] as usize..=k as usize).rev() {
                    if sum[s - nums[i] as usize] {
                        sum[s] = true;
                    }
                }
                i += 1;
            }
            if sum[k as usize] {
                ans.push(true);
            } else {
                let mut a = false;
                for j in 1..=nums.len() - i {
                    if x * j <= k as usize {
                        if sum[k as usize - x * j] {
                            a = true;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                ans.push(a);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subsequence_sum_after_capping() {
        assert_eq!(
            Solution::subsequence_sum_after_capping(vec![4, 3, 2, 4], 5),
            vec![false, false, true, true]
        );
        assert_eq!(
            Solution::subsequence_sum_after_capping(vec![1, 2, 3, 4, 5], 3),
            vec![true, true, true, true, true]
        );
    }
}
