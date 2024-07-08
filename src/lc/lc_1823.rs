// https://leetcode.com/problems/find-the-winner-of-the-circular-game/
// 1823. Find the Winner of the Circular Game
pub struct Solution;
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut v = (1..=n).collect::<Vec<i32>>();
        let mut pos = 0;
        let k = k as usize;
        while v.len() > 1 {
            pos = (pos + k - 1) % v.len();
            v.remove(pos as usize);
            if pos == v.len() {
                pos = 0;
            }
        }
        v[0]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_the_winner() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }
}
