// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/
// 1304. Find N Unique Integers Sum up to Zero
pub struct Solution;
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut n = n as usize;
        let mut v = Vec::with_capacity(n);
        if n % 2 == 1 {
            v.push(0);
            n -= 1;
        }
        for i in 1..=n / 2 {
            v.push(i as i32);
            v.push(-(i as i32));
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(n: i32) {
        let v = Solution::sum_zero(n);
        assert_eq!(v.len(), n as usize);
        let mut sum = 0;
        let mut s = std::collections::HashSet::new();
        for i in v {
            sum += i;
            assert!(s.insert(i));
        }
        assert_eq!(sum, 0);
    }
    #[test]
    fn sum_zero() {
        check(5);
        check(3);
        check(1);
    }
}
