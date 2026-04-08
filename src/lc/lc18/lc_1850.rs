// https://leetcode.com/problems/minimum-adjacent-swaps-to-reach-the-kth-smallest-number/
// 1850. Minimum Adjacent Swaps to Reach the Kth Smallest Number
pub struct Solution;
impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        fn next_permutation(nums: &mut [u8]) {
            let n = nums.len();
            if n < 2 {
                return;
            }

            let mut i = n as i32 - 2;
            while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
                i -= 1;
            }

            if i < 0 {
                nums.reverse();
                return;
            }

            let i = i as usize;
            let mut j = n - 1;
            while nums[j] <= nums[i] {
                j -= 1;
            }
            nums.swap(i, j);
            nums[i + 1..].reverse();
        }

        let original = num.into_bytes();
        let mut target = original.clone();
        for _ in 0..k {
            next_permutation(&mut target);
        }

        let mut cur = original;
        let n = cur.len();
        let mut swaps = 0_i32;

        for i in 0..n {
            if cur[i] == target[i] {
                continue;
            }

            let mut j = i + 1;
            while cur[j] != target[i] {
                j += 1;
            }

            while j > i {
                cur.swap(j - 1, j);
                swaps += 1;
                j -= 1;
            }
        }

        swaps
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_min_swaps() {
        assert_eq!(Solution::get_min_swaps("5489355142".to_string(), 4), 2);
        assert_eq!(Solution::get_min_swaps("11112".to_string(), 4), 4);
        assert_eq!(Solution::get_min_swaps("00123".to_string(), 1), 1);
    }
}
