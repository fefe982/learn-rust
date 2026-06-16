// https://leetcode.com/problems/best-poker-hand/
// 2347. Best Poker Hand
pub struct Solution;
impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        if suits.iter().all(|&s| s == suits[0]) {
            return "Flush".to_string();
        }
        let mut cnt = [0; 14];
        for rank in ranks {
            cnt[rank as usize] += 1;
        }
        if cnt.iter().any(|&c| c >= 3) {
            "Three of a Kind".to_string()
        } else if cnt.iter().any(|&c| c >= 2) {
            "Pair".to_string()
        } else {
            "High Card".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_best_hand() {
        assert_eq!(
            Solution::best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush"
        );
        assert_eq!(
            Solution::best_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
            "Three of a Kind"
        );
        assert_eq!(
            Solution::best_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "Pair"
        );
    }
}
