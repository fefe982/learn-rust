// https://leetcode.com/problems/happy-number/
// 202. Happy Number
pub struct Solution;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = std::collections::HashSet::new();
        let mut n = n;
        loop {
            if n == 1 {
                return true;
            }
            if set.contains(&n) {
                return false;
            }
            set.insert(n);
            let mut m = n;
            let mut s = 0;
            while m > 0 {
                let d = m % 10;
                s += d * d;
                m /= 10;
            }
            n = s;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_happy() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
