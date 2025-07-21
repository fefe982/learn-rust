// https://leetcode.com/problems/maximum-height-by-stacking-cuboids/
// 1691. Maximum Height by Stacking Cuboids
pub struct Solution;
impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        let mut cuboids = cuboids;
        cuboids.iter_mut().for_each(|x| x.sort());
        cuboids.sort_unstable();
        let mut max_height = cuboids.iter().map(|x| x[2]).collect::<Vec<_>>();
        let mut max = 0;
        for i in 0..cuboids.len() {
            for j in 0..i {
                if cuboids[i][0] >= cuboids[j][0] && cuboids[i][1] >= cuboids[j][1] && cuboids[i][2] >= cuboids[j][2] {
                    max_height[i] = max_height[i].max(max_height[j] + cuboids[i][2]);
                }
            }
            max = max.max(max_height[i]);
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_height() {
        assert_eq!(
            Solution::max_height(vec_vec![[50, 45, 20], [95, 37, 53], [45, 23, 12]]),
            190
        );
        assert_eq!(Solution::max_height(vec_vec![[38, 25, 45], [76, 35, 3]]), 76);
        assert_eq!(
            Solution::max_height(vec_vec![
                [7, 11, 17],
                [7, 17, 11],
                [11, 7, 17],
                [11, 17, 7],
                [17, 7, 11],
                [17, 11, 7]
            ]),
            102
        );
    }
}
