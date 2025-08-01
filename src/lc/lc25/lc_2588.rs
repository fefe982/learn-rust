// https://leetcode.com/problems/count-the-number-of-beautiful-subarrays/
// 1248. Count Number of Nice Subarrays
pub struct Solution;
impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut map = std::collections::HashMap::new();
        let mut xor = 0;
        map.insert(0, 1);
        for num in nums {
            xor ^= num;
            ans += *map.entry(xor).and_modify(|e| *e += 1).or_insert(1) as i64 - 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn beautiful_subarrays() {
        assert_eq!(Solution::beautiful_subarrays(vec![4, 3, 1, 2, 4]), 2);
        assert_eq!(Solution::beautiful_subarrays(vec![1, 10, 4]), 0);
    }
}
