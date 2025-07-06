// https://leetcode.com/problems/image-overlap/
// 835. Image Overlap
pub struct Solution;
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut one1 = vec![];
        for i in 0..img1.len() {
            for j in 0..img1[0].len() {
                if img1[i][j] == 1 {
                    one1.push((i as i32, j as i32));
                }
            }
        }
        let mut cnt = std::collections::HashMap::<(i32, i32), i32>::new();
        let mut ans = 0;
        for i in 0..img2.len() {
            for j in 0..img2[0].len() {
                if img2[i][j] == 1 {
                    for (x, y) in &one1 {
                        let val = cnt.entry((*x - i as i32, *y - j as i32)).or_insert(0);
                        *val += 1;
                        ans = ans.max(*val);
                    }
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn largest_overlap() {
        assert_eq!(
            Solution::largest_overlap(
                vec_vec![[1, 1, 0], [0, 1, 0], [0, 1, 0]],
                vec_vec![[0, 0, 0], [0, 1, 1], [0, 0, 1]]
            ),
            3
        );
        assert_eq!(Solution::largest_overlap(vec_vec![[1]], vec_vec![[1]]), 1);
        assert_eq!(Solution::largest_overlap(vec_vec![[0]], vec_vec![[0]]), 0);
    }
}
