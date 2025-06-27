// https://leetcode.com/problems/hand-of-straights/
// 846. Hand of Straights
pub struct Solution;
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        let mut m = std::collections::BTreeMap::<i32, i32>::new();
        for h in hand {
            *m.entry(h).or_default() += 1;
        }
        while m.len() > 0 {
            let first = *m.iter().next().unwrap().0;
            for i in 0..group_size as usize {
                if let Some(v) = m.get_mut(&(first + i as i32)) {
                    *v -= 1;
                    if *v == 0 {
                        m.remove(&(first + i as i32));
                    }
                } else {
                    return false;
                }
            }
        }
        return true;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_n_straight_hand() {
        assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
        assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
    }
}
