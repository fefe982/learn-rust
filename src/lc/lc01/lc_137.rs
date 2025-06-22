// https://leetcode.com/problems/single-number-ii/
// 137. Single Number II
pub struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for n in nums {
            ones ^= n & !twos;
            twos ^= n & !ones;
        }
        ones
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
