// https://leetcode.com/problems/find-the-first-player-to-win-k-games-in-a-row/
// 3175. Find The First Player to win K Games in a Row
pub struct Solution;
impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let mut pos = 0;
        let mut cnt = 0;
        let mut max = skills[0];
        for i in 1..skills.len() {
            if skills[i] > max {
                max = skills[i];
                pos = i;
                cnt = 1;
            } else {
                cnt += 1;
            }
            if cnt == k {
                break;
            }
        }
        pos as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_winning_player() {
        assert_eq!(Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
        assert_eq!(Solution::find_winning_player(vec![2, 5, 4], 3), 1);
    }
}
