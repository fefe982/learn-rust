// https://leetcode.com/problems/split-with-minimum-sum/
// 2578. Split With Minimum Sum
pub struct Solution;
impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        let mut num = num;
        let mut pow = 1;
        while num > 0 {
            h.push(std::cmp::Reverse(num % 10));
            if h.len() % 2 == 1 {
                pow *= 10;
            }
            num /= 10;
        }
        let mut sum = 0;
        pow /= 10;
        while let Some(d) = h.pop() {
            sum += d.0 * pow;
            if h.len() % 2 == 0 {
                pow /= 10;
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_num() {
        assert_eq!(Solution::split_num(4325), 59);
        assert_eq!(Solution::split_num(687), 75);
    }
}
