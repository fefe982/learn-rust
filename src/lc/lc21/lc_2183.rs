// https://leetcode.com/problems/count-array-pairs-divisible-by-k/
// 2183. Count Array Pairs Divisible by K
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            a = a % b;
            if a == 0 {
                return b;
            }
            b = b % a;
            if b == 0 {
                return a;
            }
        }
    }
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        let mut cnt = 0;
        for n in nums {
            let g = Self::gcd(n, k);
            for (&gg, &c) in m.iter() {
                if g as i64 * gg as i64 % k as i64 == 0 {
                    cnt += c as i64;
                }
            }
            *m.entry(g).or_default() += 1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_pairs() {
        assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4, 5], 2), 7);
        assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 5), 0);
    }
}
