// https://leetcode.com/problems/count-distinct-subarrays-divisible-by-k-in-sorted-array/
// 3729. Count Distinct Subarrays Divisible by K in Sorted Array
pub struct Solution;
impl Solution {
    pub fn num_good_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut cnt = std::collections::HashMap::new();
        cnt.insert(0, 1);
        let mut dcnt = std::collections::HashMap::new();
        let mut sum = 0;
        let mut ans = 0;
        let mut last = 0;
        for n in nums {
            if n != last {
                for (k, v) in dcnt.into_iter() {
                    cnt.entry(k).and_modify(|e| *e += v).or_insert(v);
                }
                dcnt = std::collections::HashMap::new();
            }
            sum = (sum + n as usize) % k;
            ans += *cnt.get(&sum).unwrap_or(&0) as i64;
            dcnt.entry(sum).and_modify(|e| *e += 1).or_insert(1);
            last = n;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_good_subarrays() {
        assert_eq!(Solution::num_good_subarrays(vec![1], 1000000000), 0);
        assert_eq!(Solution::num_good_subarrays(vec![1, 2, 3], 3), 3);
        assert_eq!(Solution::num_good_subarrays(vec![2, 2, 2, 2, 2, 2], 6), 2);
    }
}
