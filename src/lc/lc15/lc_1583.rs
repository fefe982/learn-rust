// https://leetcode.com/problems/count-unhappy-friends/
// 1583. Count Unhappy Friends
pub struct Solution;
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut p = vec![0; n];
        for pair in pairs {
            p[pair[0] as usize] = pair[1] as usize;
            p[pair[1] as usize] = pair[0] as usize;
        }
        let mut cnt = 0;
        'i: for i in 0..n {
            for j in 0..preferences[i].len() {
                let x = preferences[i][j] as usize;
                if x == p[i] {
                    break;
                }
                for k in 0..preferences[x].len() {
                    if preferences[x][k] as usize == i {
                        cnt += 1;
                        continue 'i;
                    }
                    if preferences[x][k] as usize == p[x] {
                        break;
                    }
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn unhappy_friends() {
        assert_eq!(
            Solution::unhappy_friends(
                4,
                vec_vec![[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]],
                vec_vec![[0, 1], [2, 3]]
            ),
            2
        );
        assert_eq!(Solution::unhappy_friends(2, vec_vec![[1], [0]], vec_vec![[1, 0]]), 0);
        assert_eq!(
            Solution::unhappy_friends(
                4,
                vec_vec![[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]],
                vec_vec![[1, 3], [0, 2]]
            ),
            4
        );
    }
}
