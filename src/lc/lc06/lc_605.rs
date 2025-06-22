// https://leetcode.com/problems/can-place-flowers/
// 605. Can Place Flowers
pub struct Solution;
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1;
                n -= 1;
            }
        }
        n <= 0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_place_flowers() {
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }
}
