// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/
// 1334. Find the City With the Smallest Number of Neighbors at a Threshold Distance
pub struct Solution;
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            g[i][i] = 0;
        }
        for e in edges {
            g[e[0] as usize][e[1] as usize] = e[2];
            g[e[1] as usize][e[0] as usize] = e[2];
        }
        for k in 0..n {
            for i in 0..n {
                if g[i][k] == i32::MAX {
                    continue;
                }
                for j in 0..n {
                    if g[j][k] == i32::MAX {
                        continue;
                    }
                    let d = g[i][k] + g[j][k];
                    if g[i][j] > d {
                        g[i][j] = d;
                        g[j][i] = d;
                    }
                }
            }
        }
        let mut ans = 0;
        let mut min = i32::MAX;
        for i in 0..n {
            let mut cnt = 0;
            for j in 0..n {
                if g[i][j] <= distance_threshold {
                    cnt += 1;
                }
            }
            if cnt <= min {
                ans = i;
                min = cnt;
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_the_city() {
        assert_eq!(
            Solution::find_the_city(4, vec_vec![[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]], 4),
            3
        );
        assert_eq!(
            Solution::find_the_city(
                5,
                vec_vec![[0, 1, 2], [0, 4, 8], [1, 2, 3], [1, 4, 2], [2, 3, 1], [3, 4, 1]],
                2
            ),
            0
        );
    }
}
