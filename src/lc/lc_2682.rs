// https://leetcode.com/problems/find-the-losers-of-the-circular-game/
// 2682. Find the Losers of the Circular Game
pub struct Solution;
impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut g = vec![false; n as usize];
        let mut idx = 0;
        for i in 1.. {
            if g[idx] {
                break;
            }
            g[idx] = true;
            idx += (i * k) as usize;
            idx %= n as usize;
        }
        g.into_iter()
            .zip(1..)
            .filter_map(|(b, i)| if !b { Some(i) } else { None })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circular_game_losers() {
        assert_eq!(Solution::circular_game_losers(5, 2), vec![4, 5]);
        assert_eq!(Solution::circular_game_losers(4, 4), vec![2, 3, 4]);
    }
}
