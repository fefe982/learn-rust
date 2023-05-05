// https://leetcode.com/problems/maximum-gap/description/
// 164. Maximum Gap
pub struct Solution;
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut buckets: Vec<Vec<i32>> = vec![nums];
        let mut div = 1i64;
        for _ in 0..=10 {
            let mut buckets_n: Vec<Vec<i32>> = Vec::with_capacity(10);
            for _ in 0..10 {
                buckets_n.push(Vec::new());
            }
            for vec in buckets {
                for n in vec {
                    buckets_n[(n as i64 / div % 10) as usize].push(n);
                }
            }
            div *= 10;
            buckets = buckets_n;
        }
        let mut max_diff = 0;
        for idx in 1..buckets[0].len() {
            max_diff = std::cmp::max(max_diff, buckets[0][idx] - buckets[0][idx - 1]);
        }
        max_diff
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_gap() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }
}
