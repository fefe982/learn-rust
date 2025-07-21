// https://leetcode.com/problems/maximal-network-rank/
// 1615. Maximal Network Rank
pub struct Solution;
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut g = vec![vec![0; n as usize]; n as usize];
        let mut rank = vec![0; n as usize];
        for road in roads {
            let i = road[0] as usize;
            let j = road[1] as usize;
            g[i][j] = 1;
            g[j][i] = 1;
            rank[i] += 1;
            rank[j] += 1;
        }
        let mut max_road = 0;
        for i in 0..(n - 1) as usize {
            for j in (i + 1)..(n as usize) {
                let c = rank[i] + rank[j] - g[i][j];
                if c > max_road {
                    max_road = c;
                }
            }
        }
        max_road
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximal_network_rank() {
        assert_eq!(
            Solution::maximal_network_rank(4, vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]]),
            4
        );
        assert_eq!(
            Solution::maximal_network_rank(
                5,
                vec![
                    vec![0, 1],
                    vec![0, 3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 3],
                    vec![2, 4]
                ]
            ),
            5
        );
        assert_eq!(
            Solution::maximal_network_rank(
                8,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![2, 4],
                    vec![5, 6],
                    vec![5, 7]
                ]
            ),
            5
        );
    }
}
