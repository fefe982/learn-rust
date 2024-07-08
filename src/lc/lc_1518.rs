// https://leetcode.com/problems/water-bottles/
// 1518. Water Bottles
pub struct Solution;
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total = num_bottles;
        let mut empty_bottles = num_bottles;
        while empty_bottles >= num_exchange {
            let new_bottles = empty_bottles / num_exchange;
            total += new_bottles;
            empty_bottles = empty_bottles % num_exchange + new_bottles;
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_water_bottles() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
