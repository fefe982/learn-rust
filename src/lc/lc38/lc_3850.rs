// https://leetcode.com/problems/count-sequences-to-k/
// 3850. Count Sequences to K
pub struct Solution;
impl Solution {
    fn count(
        nums: &Vec<i32>,
        idx: usize,
        k2: i32,
        k3: i32,
        k5: i32,
        cache: &mut std::collections::HashMap<(usize, i32, i32, i32), i32>,
    ) -> i32 {
        if idx == nums.len() {
            if k2 == 0 && k3 == 0 && k5 == 0 {
                return 1;
            }
            return 0;
        }
        let key = (idx, k2, k3, k5);
        if let Some(&v) = cache.get(&key) {
            return v;
        }
        let mut ans = 0;
        let mut d2 = 0;
        let mut d3 = 0;
        let mut d5 = 0;
        match nums[idx] {
            1 => {}
            2 => d2 = 1,
            3 => d3 = 1,
            4 => d2 = 2,
            5 => d5 = 1,
            6 => {
                d2 = 1;
                d3 = 1;
            }
            _ => {}
        }
        ans += Self::count(nums, idx + 1, k2 - d2, k3 - d3, k5 - d5, cache)
            + Self::count(nums, idx + 1, k2, k3, k5, cache)
            + Self::count(nums, idx + 1, k2 + d2, k3 + d3, k5 + d5, cache);
        cache.insert(key, ans);
        ans
    }
    pub fn count_sequences(nums: Vec<i32>, k: i64) -> i32 {
        let mut k2 = 0;
        let mut k3 = 0;
        let mut k5 = 0;
        let mut k = k;
        while k % 2 == 0 {
            k2 += 1;
            k /= 2;
        }
        while k % 3 == 0 {
            k3 += 1;
            k /= 3;
        }
        while k % 5 == 0 {
            k5 += 1;
            k /= 5;
        }
        if k != 1 {
            return 0;
        }
        Self::count(&nums, 0, k2, k3, k5, &mut std::collections::HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_sequences() {
        assert_eq!(Solution::count_sequences(vec![2, 3, 2], 6), 2);
        assert_eq!(Solution::count_sequences(vec![4, 6, 3], 2), 2);
        assert_eq!(Solution::count_sequences(vec![1, 5], 1), 3);
    }
}
