// https://leetcode.com/problems/water-bottles-ii/
// 3100. Water Bottles II
pub struct Solution;
impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut sum = num_bottles;
        let mut cur = num_bottles;
        let mut num_exchange = num_exchange;
        while cur >= num_exchange {
            cur -= num_exchange - 1;
            sum += 1;
            num_exchange += 1;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_bottles_drunk() {
        assert_eq!(Solution::max_bottles_drunk(13, 6), 15);
        assert_eq!(Solution::max_bottles_drunk(10, 3), 13);
    }
}
