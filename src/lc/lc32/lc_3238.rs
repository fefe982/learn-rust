// https://leetcode.com/problems/find-the-number-of-winning-players/
// 3238. Find the Number of Winning Players
pub struct Solution;
impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![vec![0; 11]; n as usize + 1];
        for p in pick {
            cnt[p[0] as usize][p[1] as usize] += 1;
        }
        cnt.into_iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if c.into_iter().filter(|&x| x as usize > i).count() > 0 {
                    Some(i)
                } else {
                    None
                }
            })
            .count() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_winning_player_count() {
        assert_eq!(
            Solution::winning_player_count(6, vec_vec![[0, 0], [1, 0], [1, 0], [2, 1], [2, 1], [2, 0]]),
            2
        );
        assert_eq!(
            Solution::winning_player_count(5, vec_vec![[1, 1], [1, 2], [1, 3], [1, 4]]),
            0
        )
    }
}
