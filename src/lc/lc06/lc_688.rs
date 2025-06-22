// https://leetcode.com/problems/knight-probability-in-chessboard/
// 688. Knight Probability in Chessboard
pub struct Solution;
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut offboard = 0.0;
        let n = n as usize;
        let mut prob = vec![vec![0.0; n]; n];
        prob[row as usize][column as usize] = 1.0;
        let dir = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];
        for _ in 0..k {
            let mut nprob = vec![vec![0.0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    if prob[i][j] > 0.0 {
                        let padd = prob[i][j] / 8.0;
                        for (di, dj) in dir {
                            let ni = (i as i32 + di) as usize;
                            let nj = (j as i32 + dj) as usize;
                            if ni >= n || nj >= n {
                                offboard += padd;
                            } else {
                                nprob[ni][nj] += padd;
                            }
                        }
                    }
                }
            }
            prob = nprob;
        }
        1.0 - offboard
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn knight_probability() {
        assert_eq!(Solution::knight_probability(3, 2, 0, 0), 0.0625);
        assert_eq!(Solution::knight_probability(1, 0, 0, 0), 1.0);
    }
}
