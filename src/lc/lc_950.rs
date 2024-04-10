// https://leetcode.com/problems/reveal-cards-in-increasing-order/
// 950. Reveal Cards In Increasing Order
pub struct Solution;
impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = deck;
        deck.sort_unstable();
        let mut ans = vec![0; deck.len()];
        let mut l = deck.len() - 1;
        for i in 0..deck.len() {
            if i % 2 == 0 {
                ans[i] = deck[i / 2];
            } else {
                let mut ll = l;
                let mut idx = i;
                while ll % 2 == 0 {
                    idx += ll;
                    ll /= 2;
                }
                ans[i] = deck[(idx + ll) / 2];
                l -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deck_revealed_increasing() {
        assert_eq!(
            Solution::deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
            [2, 13, 3, 11, 5, 17, 7]
        );
        assert_eq!(Solution::deck_revealed_increasing(vec![1, 1000]), [1, 1000]);
    }
}
