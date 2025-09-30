// https://leetcode.cn/problems/closest-divisors/description/
// 1362. Closest Divisors
pub struct Solution;
impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let mut n = (num as f64).sqrt() as i32 + 1;
        loop {
            if (num + 1) % n == 0 {
                return vec![n, (num + 1) / n];
            } else if (num + 2) % n == 0 {
                return vec![n, (num + 2) / n];
            }
            n -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn closest_divisors() {
        assert_sort_eq!(Solution::closest_divisors(8), vec![3, 3]);
        assert_sort_eq!(Solution::closest_divisors(123), vec![5, 25]);
        assert_sort_eq!(Solution::closest_divisors(999), vec![40, 25]);
    }
}
