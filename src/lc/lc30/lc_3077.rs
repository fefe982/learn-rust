// https://leetcode.com/problems/maximum-strength-of-k-disjoint-subarrays/
// 3077. Maximum Strength of K Disjoint Subarrays
pub struct Solution;
impl Solution {
    fn select(nums: &[i32], k: usize, sep: usize, mut cache: &mut Vec<Vec<Vec<i64>>>) -> i64 {
        let len = nums.len();
        if k == 0 || len == 0 {
            return 0;
        }
        if cache[len][k][sep] != i64::MIN {
            return cache[len][k][sep];
        }
        let mut max = i64::MIN;
        let mul = if k % 2 == 1 { 1 } else { -1 };
        if k <= len {
            if sep == 0 {
                max = max.max(Self::select(&nums[1..], k, sep, cache) + mul * nums[0] as i64 * k as i64);
            }
            max = max.max(Self::select(&nums[1..], k, 1, cache));
        }
        max = max.max(Self::select(&nums[1..], k - 1, 0, &mut cache) - mul * nums[0] as i64 * (k - 1) as i64);
        cache[len][k][sep] = max;
        max
    }
    pub fn maximum_strength(nums: Vec<i32>, k: i32) -> i64 {
        Self::select(
            &nums[..],
            k as usize + 1,
            1,
            &mut vec![vec![vec![i64::MIN; 2]; k as usize + 2]; nums.len() + 1],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_strength() {
        assert_eq!(Solution::maximum_strength(vec![1, 2, 3, -1, 2], 3), 22);
        assert_eq!(Solution::maximum_strength(vec![12, -2, -2, -2, -2], 5), 64);
        assert_eq!(Solution::maximum_strength(vec![-1, -2, -3], 1), -1);
    }
}
