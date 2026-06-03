// https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/
// 2260. Minimum Consecutive Cards to Pick Up
pub struct Solution;
impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut last_pos = std::collections::HashMap::new();
        let mut ans = i32::MAX;
        for (i, &card) in cards.iter().enumerate() {
            if let Some(&pos) = last_pos.get(&card) {
                ans = ans.min((i - pos + 1) as i32);
            }
            last_pos.insert(card, i);
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_card_pickup() {
        assert_eq!(Solution::minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]), 4);
        assert_eq!(Solution::minimum_card_pickup(vec![1, 0, 5, 3]), -1);
    }
}
