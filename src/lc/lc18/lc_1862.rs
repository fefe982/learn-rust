// https://leetcode.com/problems/sum-of-floored-pairs/
// 1862. Sum of Floored Pairs
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        let &max = nums.iter().max().unwrap();
        let mut cnt = vec![0; max as usize + 1];
        for n in nums.into_iter() {
            cnt[n as usize] += 1;
        }
        let mut sum = cnt.clone();
        for i in 1..=max as usize {
            sum[i] += sum[i - 1];
        }
        let mut ans = 0;
        for i in 1..=max as usize {
            if cnt[i] == 0 {
                continue;
            }
            let mut sum_low = sum[i - 1];
            for j in 1.. {
                let high = (i * (j + 1) - 1).min(max as usize);
                ans = (ans + ((cnt[i] as i64 * j as i64) % MOD * (sum[high] - sum_low)) % MOD) % MOD;
                if high == max as usize {
                    break;
                }
                sum_low = sum[high];
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_floored_pairs() {
        assert_eq!(Solution::sum_of_floored_pairs(vec![2, 5, 9]), 10);
        assert_eq!(Solution::sum_of_floored_pairs(vec![7, 7, 7, 7, 7, 7, 7]), 49);
    }
}
