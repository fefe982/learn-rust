// https://leetcode.com/problems/reordered-power-of-2/
// 869. Reordered Power of 2
pub struct Solution;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut h = std::collections::HashSet::new();
        fn count(n: i32) -> [u8; 10] {
            let mut c = [0; 10];
            let mut n = n;
            while n > 0 {
                c[(n % 10) as usize] += 1;
                n /= 10;
            }
            c
        }
        for i in 0..=30 {
            h.insert(count(1 << i));
        }
        h.contains(&count(n))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reordered_power_of2() {
        assert_eq!(Solution::reordered_power_of2(1), true);
        assert_eq!(Solution::reordered_power_of2(10), false);
        assert_eq!(Solution::reordered_power_of2(16), true);
        assert_eq!(Solution::reordered_power_of2(24), false);
    }
}
