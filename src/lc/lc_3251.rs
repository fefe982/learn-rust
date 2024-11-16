// https://leetcode.com/problems/find-the-count-of-monotonic-pairs-ii/
// 3251. Find the Count of Monotonic Pairs in an Array
pub struct Solution;
impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i64;
        let l = nums.windows(2).map(|v| (v[1] - v[0]).max(0) as i64).sum::<i64>();
        let r = nums[len as usize - 1] as i64;
        if l > r {
            return 0;
        }
        let mut num = 1;
        let mut denom = 1;
        let m = 1_000_000_007;
        let gap = r - l;
        for i in 1..=len {
            num = (num * (gap + i)) % m;
            denom = (denom * i) % m;
        }
        while num % denom != 0 {
            let n = m / denom;
            num = (num * (n + 1)) % m;
            denom = (denom * (n + 1)) % m;
        }
        (num / denom) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_of_pairs() {
        assert_eq!(Solution::count_of_pairs(vec![2, 3, 2]), 4);
        assert_eq!(Solution::count_of_pairs(vec![5, 5, 5, 5]), 126);
    }
}
