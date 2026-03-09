// https://leetcode.com/problems/ways-to-split-array-into-three-subarrays/
// 1712. Ways to Split Array Into Three Subarrays
pub struct Solution;
impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        let mut k = 1;
        let mut j = 1;
        let all = nums[nums.len() - 1];
        let mut ans = 0;
        for i in 0..nums.len() - 2 {
            if j == i {
                j += 1
            }
            while j + 1 < nums.len() - 1 && nums[j] - nums[i] < nums[i] {
                j += 1;
            }
            if nums[j] - nums[i] < nums[i] {
                break;
            }
            while k + 1 < nums.len() - 1 && all - nums[k + 1] >= nums[k + 1] - nums[i] {
                k += 1;
            }
            if k >= j && all - nums[k] >= nums[k] - nums[i] {
                ans += (k - j + 1) as i64;
            }
        }
        (ans % 1_000_000_007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ways_to_split() {
        assert_eq!(Solution::ways_to_split(vec![0, 0, 0, 0]), 3);
        assert_eq!(
            Solution::ways_to_split(vec![
                8892, 2631, 7212, 1188, 6580, 1690, 5950, 7425, 8787, 4361, 9849, 4063, 9496, 9140, 9986, 1058, 2734,
                6961, 8855, 2567, 7683, 4770, 40, 850, 72, 2285, 9328, 6794, 8632, 9163, 3928, 6962, 6545, 6920, 926,
                8885, 1570, 4454, 6876, 7447, 8264, 3123, 2980, 7276, 470, 8736, 3153, 3924, 3129, 7136, 1739, 1354,
                661, 1309, 6231, 9890, 58, 4623, 3555, 3100, 3437
            ]),
            227
        );
        assert_eq!(Solution::ways_to_split(vec![1, 1, 1]), 1);
        assert_eq!(Solution::ways_to_split(vec![1, 2, 2, 2, 5, 0]), 3);
        assert_eq!(Solution::ways_to_split(vec![3, 2, 1]), 0);
    }
}
