// https://leetcode.com/problems/min-cost-to-connect-all-points/
// 1584. Min Cost to Connect All Points
pub struct Solution;
impl Solution {
    fn get_p(p: &Vec<usize>, mut i: usize) -> usize {
        while p[i] != i {
            i = p[i];
        }
        i
    }
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut hp = std::collections::BinaryHeap::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                hp.push((
                    std::cmp::Reverse(
                        (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(),
                    ),
                    i,
                    j,
                ));
            }
        }
        let mut p: Vec<usize> = (0..points.len()).collect();
        let mut sum = 0;
        while let Some((d, i, j)) = hp.pop() {
            let pi = Self::get_p(&p, i);
            let pj = Self::get_p(&p, j);
            if pi == pj {
                continue;
            }
            p[pi] = pj;
            p[i] = pj;
            sum += d.0;
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_cost_connect_points() {
        assert_eq!(
            Solution::min_cost_connect_points(vec_vec![[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]),
            20
        );
        assert_eq!(
            Solution::min_cost_connect_points(vec_vec![[3, 12], [-2, 5], [-4, 1]]),
            18
        );
    }
}
