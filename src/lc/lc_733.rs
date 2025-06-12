// https://leetcode.com/problems/flood-fill/
// 733. Flood Fill
pub struct Solution;
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let mut image = image;
        let src = image[sr][sc];
        if src == color {
            return image;
        }
        let mut stack = vec![(sr, sc)];
        image[sr][sc] = color;
        while let Some((r, c)) = stack.pop() {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (r, c) = ((r as i32 + dr) as usize, (c as i32 + dc) as usize);
                if r < image.len() && c < image[0].len() && image[r][c] == src {
                    stack.push((r, c));
                    image[r][c] = color;
                }
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
    fn flood_fill() {
        assert_eq!(
            Solution::flood_fill(vec_vec![[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 2),
            vec_vec![[2, 2, 2], [2, 2, 0], [2, 0, 1]]
        );
        assert_eq!(
            Solution::flood_fill(vec_vec![[0, 0, 0], [0, 0, 0]], 0, 0, 0),
            vec_vec![[0, 0, 0], [0, 0, 0]]
        );
    }
}
