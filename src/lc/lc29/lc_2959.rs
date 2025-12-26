// https://leetcode.com/problems/number-of-possible-sets-of-closing-branches/
// 2959. Number of Possible Sets of Closing Branches
pub struct Solution;
impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![i32::MAX / 2; n]; n];
        for i in 0..n {
            graph[i][i] = 0;
        }
        for road in roads {
            graph[road[0] as usize][road[1] as usize] = graph[road[0] as usize][road[1] as usize].min(road[2]);
            graph[road[1] as usize][road[0] as usize] = graph[road[1] as usize][road[0] as usize].min(road[2]);
        }
        let mut nums = 0;
        for mask in 0..(1 << n) {
            let mut g = graph.clone();
            for k in 0..n {
                if mask & (1 << k) == 0 {
                    continue;
                }
                for i in 0..n {
                    if (mask & (1 << i)) == 0 {
                        continue;
                    }
                    for j in 0..n {
                        if (mask & (1 << j)) == 0 {
                            continue;
                        }
                        g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
                    }
                }
            }
            let mut max = 0;
            for i in 0..n {
                if (mask & (1 << i)) == 0 {
                    continue;
                }
                for j in 0..n {
                    if (mask & (1 << j)) == 0 {
                        continue;
                    }
                    max = max.max(g[i][j]);
                }
            }
            if max <= max_distance {
                nums += 1;
            }
        }
        nums
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_sets() {
        assert_eq!(
            Solution::number_of_sets(3, 5, vec_vec![[0, 1, 2], [1, 2, 10], [0, 2, 10]]),
            5
        );
        assert_eq!(
            Solution::number_of_sets(3, 5, vec_vec![[0, 1, 20], [0, 1, 10], [1, 2, 2], [0, 2, 2]]),
            7
        );
        assert_eq!(Solution::number_of_sets(1, 10, vec_vec![]), 2);
    }
}
