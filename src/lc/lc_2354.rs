// https://leetcode.com/problems/number-of-excellent-pairs/
// 2354. Number of Excellent Pairs
pub struct Solution;
impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = vec![0; 31];
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            cnt[nums[i].count_ones() as usize] += 1;
        }
        for i in 1..31 {
            cnt[i] += cnt[i - 1];
        }
        let mut ans = 0;
        for i in 1..31 {
            let mut other = (k - i).max(i);
            let c = (cnt[i as usize] - cnt[i as usize - 1]) as i64;
            if other == i {
                ans += c * c;
                other += 1;
            }
            if other < 31 {
                ans += c * (cnt[30] - cnt[other as usize - 1]) as i64 * 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_excellent_pairs() {
        assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 1, 536870911], 3), 12);
        assert_eq!(Solution::count_excellent_pairs(vec![1, 2, 3, 1], 3), 5);
        assert_eq!(Solution::count_excellent_pairs(vec![5, 1, 1, 2, 2, 3, 3, 5], 10), 0);
    }
}
