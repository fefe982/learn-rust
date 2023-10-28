// https://leetcode.com/problems/find-the-punishment-number-of-an-integer/
// 2698. Find the Punishment Number of an Integer
pub struct Solution;
impl Solution {
    fn check(n: i32, sum: i32) -> bool {
        if n == sum {
            return true;
        }
        if sum < 0 {
            return false;
        }
        let mut pow = 10;
        while n > pow {
            if Solution::check(n / pow, sum - n % pow) {
                return true;
            }
            pow *= 10;
        }
        false
    }
    pub fn punishment_number(n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..=n {
            if Solution::check(i * i, i) {
                sum += i * i;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_punishment_number() {
        assert_eq!(Solution::punishment_number(10), 182);
        assert_eq!(Solution::punishment_number(37), 1478);
    }
}
