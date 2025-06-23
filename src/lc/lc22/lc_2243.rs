// https://leetcode.com/problems/maximum-score-of-a-node-sequence/
// 2242. Maximum Score of a Node Sequence
pub struct Solution;
impl Solution {
    fn set_edge(from: &mut Vec<(i32, i32)>, to: i32, score: i32) {
        for i in (0..3).rev() {
            let insert = (to, score);
            if insert.1 > from[i].1 {
                if i < 2 {
                    from[i + 1] = from[i];
                }
                if i == 0 {
                    from[i] = insert;
                }
            } else {
                if i < 2 {
                    from[i + 1] = insert;
                }
                break;
            }
        }
    }
    pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = scores.len();
        let mut g = vec![vec![(n as i32, 0); 3]; n];
        for e in &edges {
            Self::set_edge(&mut g[e[0] as usize], e[1], scores[e[1] as usize]);
            Self::set_edge(&mut g[e[1] as usize], e[0], scores[e[0] as usize]);
        }
        let mut max = -1;
        for e in &edges {
            let mut n0 = 0;
            if g[e[0] as usize][0].0 == e[1] {
                n0 += 1;
            }
            if g[e[0] as usize][n0].1 == 0 {
                continue;
            }
            let mut n1 = 0;
            if g[e[1] as usize][0].0 == e[0] {
                n1 += 1;
            }
            if g[e[1] as usize][n1].1 == 0 {
                continue;
            }
            if g[e[0] as usize][n0].0 != g[e[1] as usize][n1].0 {
                max = max.max(
                    scores[e[0] as usize] + scores[e[1] as usize] + g[e[0] as usize][n0].1 + g[e[1] as usize][n1].1,
                );
            } else {
                let score0 = g[e[0] as usize][n0].1;
                n0 += 1;
                if g[e[0] as usize][n0].0 == e[1] {
                    n0 += 1;
                }
                n1 += 1;
                if g[e[1] as usize][n1].0 == e[0] {
                    n1 += 1;
                }
                let score1 = g[e[0] as usize][n0].1.max(g[e[1] as usize][n1].1);
                if score1 == 0 {
                    continue;
                }
                max = max.max(scores[e[0] as usize] + scores[e[1] as usize] + score0 + score1);
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_score() {
        assert_eq!(
            Solution::maximum_score(
                vec![18, 6, 4, 9, 8, 2],
                vec_vec![
                    [0, 1],
                    [0, 2],
                    [0, 3],
                    [0, 4],
                    [0, 5],
                    [1, 2],
                    [1, 3],
                    [1, 4],
                    [1, 5],
                    [2, 3],
                    [2, 4],
                    [2, 5],
                    [3, 4],
                    [3, 5],
                    [4, 5]
                ]
            ),
            41
        );
        assert_eq!(
            Solution::maximum_score(
                vec![5, 2, 9, 8, 4],
                vec_vec![[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]]
            ),
            24
        );
        assert_eq!(
            Solution::maximum_score(vec![9, 20, 6, 4, 11, 12], vec_vec![[0, 3], [5, 3], [2, 4], [1, 3]]),
            -1
        );
    }
}
