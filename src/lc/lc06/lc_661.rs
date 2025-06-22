// https://leetcode.com/problems/image-smoother/
// 661. Image Smoother
pub struct Solution;
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();
        let mut res = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                let mut cnt = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let x = (i as i32 + di) as usize;
                        let y = (j as i32 + dj) as usize;
                        if x < m && y < n {
                            sum += img[x][y];
                            cnt += 1;
                        }
                    }
                }
                res[i][j] = sum / cnt;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_image_smoother() {
        assert_eq!(
            Solution::image_smoother(vec_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]]),
            vec_vec![[0, 0, 0], [0, 0, 0], [0, 0, 0]]
        );
        assert_eq!(
            Solution::image_smoother(vec_vec![[100, 200, 100], [200, 50, 200], [100, 200, 100]]),
            vec_vec![[137, 141, 137], [141, 138, 141], [137, 141, 137]]
        );
    }
}
