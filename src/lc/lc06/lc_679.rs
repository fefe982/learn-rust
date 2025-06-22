// https://leetcode.com/problems/24-game/
// 679. 24 Game
pub struct Solution;
impl Solution {
    fn binary(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
        vec![
            (x1 * y2 + x2 * y1, y1 * y2),
            (x1 * y2 - x2 * y1, y1 * y2),
            (x1 * x2, y1 * y2),
            (x1 * y2, y1 * x2),
        ]
    }
    fn reduce_point24(x: i32, y: i32, cards: &[i32]) -> bool {
        if cards.len() == 0 {
            return y != 0 && x % y == 0 && x / y == 24;
        }
        if cards.len() == 2 {
            for (x2, y2) in Self::binary(cards[0], 1, cards[1], 1) {
                for (x3, y3) in Self::binary(x, y, x2, y2) {
                    if y3 != 0 && x3 % y3 == 0 && x3 / y3 == 24 {
                        return true;
                    }
                }
            }
        }
        return Self::reduce_point24(x + y * cards[0], y, &cards[1..])
            || Self::reduce_point24(x - y * cards[0], y, &cards[1..])
            || Self::reduce_point24(y * cards[0] - x, y, &cards[1..])
            || Self::reduce_point24(x * cards[0], y, &cards[1..])
            || Self::reduce_point24(x, y * cards[0], &cards[1..])
            || Self::reduce_point24(y * cards[0], x, &cards[1..]);
    }
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let mut cards = cards;
        cards.sort_unstable();
        let mut h = vec![0; cards.len()];
        loop {
            if Self::reduce_point24(cards[0], 1, &cards[1..]) {
                return true;
            }

            let mut max = 0;
            let mut anchor = 0;
            let mut anchor_pos = 0;
            for (i, &c) in cards.iter().rev().enumerate() {
                if c < max {
                    anchor = c;
                    anchor_pos = cards.len() - i - 1;
                    break;
                } else {
                    h[i] = c;
                    max = c;
                }
            }
            if anchor == 0 {
                break;
            }
            let mut p = anchor_pos + 1;
            for i in 0..(cards.len() - anchor_pos - 1) {
                let c = h[i];
                if c <= anchor {
                    cards[p] = c;
                } else {
                    cards[anchor_pos] = c;
                    cards[p] = anchor;
                    anchor = i32::MAX;
                }
                p += 1;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_judge_point24() {
        assert_eq!(Solution::judge_point24(vec![1, 3, 4, 6]), true);
        assert_eq!(Solution::judge_point24(vec![4, 1, 8, 7]), true);
        assert_eq!(Solution::judge_point24(vec![1, 2, 1, 2]), false);
    }
}
