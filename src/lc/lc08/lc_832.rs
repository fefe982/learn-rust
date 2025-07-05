// https://leetcode.com/problems/flipping-an-image/
// 832. Flipping an Image
pub struct Solution;
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut image = image;
        let n = image[0].len();
        for row in image.iter_mut() {
            for i in 0..n / 2 {
                if row[i] == row[n - i - 1] {
                    row[i] ^= 1;
                    row[n - i - 1] ^= 1;
                }
            }
            if n % 2 == 1 {
                row[n / 2] ^= 1;
            }
        }
        image
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn flip_and_invert_image() {
        assert_eq!(
            Solution::flip_and_invert_image(vec_vec![[1, 1, 0], [1, 0, 1], [0, 0, 0]]),
            vec_vec![[1, 0, 0], [0, 1, 0], [1, 1, 1]]
        );
        assert_eq!(
            Solution::flip_and_invert_image(vec_vec![[1, 1, 0, 0], [1, 0, 0, 1], [0, 1, 1, 1], [1, 0, 1, 0]]),
            vec_vec![[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 1], [1, 0, 1, 0]]
        );
    }
}
