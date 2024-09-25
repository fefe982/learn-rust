// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares/
// 2897. Apply Operations on Array to Maximize Sum of Squares
pub struct Solution;
impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; 31];
        for n in nums {
            let mut n = n;
            while n > 0 {
                cnt[n.trailing_zeros() as usize] += 1;
                n &= n - 1;
            }
        }
        let mut sum = 0;
        for _ in 0..k {
            let mut n = 0;
            for i in 0..31 {
                if cnt[i] > 0 {
                    n += 1 << i;
                    cnt[i] -= 1;
                }
            }
            sum = (sum + n * n) % 1_000_000_007i64;
        }
        sum as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_sum() {
        assert_eq!(Solution::max_sum(vec![2, 6, 5, 8], 2), 261);
        assert_eq!(Solution::max_sum(vec![4, 5, 4, 7], 3), 90);
    }
}
