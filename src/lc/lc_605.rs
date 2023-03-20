// https://leetcode.com/problems/can-place-flowers/
// 605. Can Place Flowers
pub struct Solution;
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flower = 0;
        let mut last_flower = 0usize;
        for (idx, f) in [1, 0]
            .iter()
            .chain(flowerbed.iter())
            .chain([0, 1].iter())
            .enumerate()
        {
            if *f == 1 {
                flower += ((std::cmp::max(idx - last_flower, 1usize) - 1) / 2) as i32;
                last_flower = idx + 1;
            }
        }
        flower >= n
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
