// https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/
// 914. X of a Kind in a Deck of Cards
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if a == 0 {
                return b;
            }
            b = b % a;
            if b == 0 {
                return a;
            }
            a = a % b;
        }
    }
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut deck = deck;
        deck.sort_unstable();
        deck.push(-1);
        let mut gcd = 0;
        let mut cnt = 1;
        for i in 1..deck.len() {
            if deck[i] == deck[i - 1] {
                cnt += 1;
            } else {
                gcd = Self::gcd(gcd, cnt);
                cnt = 1;
            }
        }
        gcd > 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_groups_size_x() {
        assert_eq!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]), true);
        assert_eq!(Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]), false);
    }
}
