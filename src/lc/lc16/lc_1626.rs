// https://leetcode.com/problems/best-team-with-no-conflicts/
// 1626. Best Team With No Conflicts
pub struct Solution;
impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let mut players: Vec<(i32, i32)> =
            ages.iter().cloned().zip(scores.iter().cloned()).collect();
        players.sort();
        let mut max_score = vec![0; players.len()];
        let mut max_result = 0;
        for idx in 0..players.len() {
            let (age, score) = players[idx];
            if max_score[idx] < score {
                max_score[idx] = score
            }
            for idx_prev in 0..idx {
                if (players[idx_prev].0 == age || players[idx_prev].1 <= score)
                    && max_score[idx_prev] + score > max_score[idx]
                {
                    max_score[idx] = max_score[idx_prev] + score;
                }
            }
            if max_score[idx] > max_result {
                max_result = max_score[idx]
            }
        }
        max_result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn best_team_score() {
        assert_eq!(
            Solution::best_team_score(vec![1, 3, 5, 10, 15], vec![1, 2, 3, 4, 5]),
            34
        );
        assert_eq!(
            Solution::best_team_score(vec![4, 5, 6, 5], vec![2, 1, 2, 1]),
            16
        );
        assert_eq!(
            Solution::best_team_score(vec![1, 2, 3, 5], vec![8, 9, 10, 1]),
            6
        );
    }
}
