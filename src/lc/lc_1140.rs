// https://leetcode.com/problems/stone-game-ii/
// 1140. Stone Game II
pub struct Solution;
impl Solution {
    fn stone(
        piles: &[i32],
        m: usize,
        cache: &mut std::collections::HashMap<(usize, usize), (i32, i32)>,
    ) -> (i32, i32) {
        let m = std::cmp::min((piles.len() + 1) / 2, m);
        if let Some(&v) = cache.get(&(piles.len(), m)) {
            return v;
        }
        let mut sum = 0;
        let mut max = 0;
        let mut other = 0;
        for i in 0..m * 2 {
            if i >= piles.len() {
                break;
            }
            sum += piles[i];
            let rest = Self::stone(&piles[i + 1..], std::cmp::max(i + 1, m), cache);
            if sum + rest.1 > max {
                max = sum + rest.1;
                other = rest.0;
            }
        }
        cache.insert((piles.len(), m), (max, other));
        (max, other)
    }
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        Self::stone(&piles[..], 1, &mut std::collections::HashMap::new()).0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stone_game_ii() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }
}
