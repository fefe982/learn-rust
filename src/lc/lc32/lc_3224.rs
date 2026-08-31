// https://leetcode.com/problems/minimum-array-changes-to-make-differences-equal/
// 3224. Minimum Array Changes to Make Differences Equal
pub struct Solution;
impl Solution {
    pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; k as usize + 2];
        cnt[0] = nums.len() as i32 / 2;
        for i in 0..nums.len() / 2 {
            let a = nums[i];
            let b = nums[nums.len() - 1 - i];
            let diff = (a - b).abs();
            let max_diff = a.max(b).max(k - a).max(k - b);
            cnt[diff as usize] -= 1;
            cnt[diff as usize + 1] += 1;
            cnt[max_diff as usize + 1] += 1;
        }
        let mut ans = cnt[0];
        let mut s = cnt[0];
        for i in 1..cnt.len() - 1 {
            s += cnt[i];
            ans = ans.min(s);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_changes() {
        assert_eq!(Solution::min_changes(vec![1, 0, 1, 2, 4, 3], 4), 2);
        assert_eq!(Solution::min_changes(vec![0, 1, 2, 3, 3, 6, 5, 4], 6), 2);
    }
}
