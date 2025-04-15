// https://leetcode.com/problems/count-the-number-of-good-subarrays/
// 2537. Count the Number of Good Subarrays
pub struct Solution;
impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        let mut i = 0;
        let mut p = 0;
        let mut res = 0;
        for j in 0..nums.len() {
            let c = m.entry(nums[j]).or_insert(0);
            p += *c;
            *c += 1;
            while p >= k {
                res += (nums.len() - j) as i64;
                let c = m.entry(nums[i]).or_insert(0);
                *c -= 1;
                p -= *c;
                i += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_good() {
        assert_eq!(Solution::count_good(vec![1, 1, 1, 1, 1], 10), 1);
        assert_eq!(Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
    }
}
