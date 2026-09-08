// https://leetcode.com/problems/maximum-points-tourist-can-earn/
// 3322. Maximum Points Tourist Can Earn
pub struct Solution;
impl Solution {
    pub fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut score = vec![0; n];
        for i in 0..k {
            let mut nscore = vec![0; n];
            for j in 0..n {
                for k in 0..n {
                    if j == k {
                        nscore[k] = nscore[k].max(score[j] + stay_score[i][j]);
                    } else {
                        nscore[k] = nscore[k].max(score[j] + travel_score[j][k]);
                    }
                }
            }
            score = nscore;
        }
        score.iter().max().unwrap().clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_score() {
        assert_eq!(Solution::max_score(2, 1, vec_vec![[2, 3]], vec_vec![[0, 2], [1, 0]]), 3);
        assert_eq!(
            Solution::max_score(
                3,
                2,
                vec_vec![[3, 4, 2], [2, 1, 2]],
                vec_vec![[0, 2, 1], [2, 0, 4], [3, 2, 0]]
            ),
            8
        );
    }
}
