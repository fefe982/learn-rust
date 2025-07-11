// https://leetcode.com/problems/average-value-of-even-numbers-that-are-divisible-by-three/
// 2455. Average Value of Even Numbers That Are Divisible by Three
pub struct Solution;
impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut cnt = 0;
        nums.iter().for_each(|&x| {
            if x % 6 == 0 {
                sum += x;
                cnt += 1;
            }
        });
        if cnt == 0 {
            0
        } else {
            sum / cnt
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn average_value() {
        assert_eq!(Solution::average_value(vec![1, 3, 6, 10, 12, 15]), 9);
        assert_eq!(Solution::average_value(vec![1, 2, 4, 7, 10]), 0);
    }
}
