// https://leetcode.com/problems/card-flipping-game/
// 822. Card Flipping Game
pub struct Solution;
impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut h = std::collections::BinaryHeap::new();
        let mut c = std::collections::HashSet::new();
        for i in 0..fronts.len() {
            if fronts[i] != backs[i] {
                h.push(std::cmp::Reverse(fronts[i]));
                h.push(std::cmp::Reverse(backs[i]));
            } else {
                c.insert(fronts[i]);
            }
        }
        while let Some(n) = h.pop() {
            if !c.contains(&n.0) {
                return n.0;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flipgame() {
        assert_eq!(
            Solution::flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3]),
            2
        );
        assert_eq!(Solution::flipgame(vec![1], vec![1]), 0);
    }
}
