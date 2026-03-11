// https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/
// 1725. Number Of Rectangles That Can Form The Largest Square
pub struct Solution;
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let max = rectangles.iter().map(|r| r[0].min(r[1])).max().unwrap();
        rectangles.iter().filter(|r| r[0].min(r[1]) == max).count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_good_rectangles() {
        assert_eq!(
            Solution::count_good_rectangles(vec_vec![[5, 8], [3, 9], [5, 12], [16, 5]]),
            3
        );
        assert_eq!(
            Solution::count_good_rectangles(vec_vec![[2, 3], [3, 7], [4, 3], [3, 7]]),
            3
        );
    }
}
