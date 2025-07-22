// https://leetcode.com/problems/array-of-doubled-pairs/
// 954. Array of Doubled Pairs
use std::collections::BTreeMap;
pub struct Solution;
impl Solution {
    fn check_map(map: &mut BTreeMap<i32, i32>) -> bool {
        let keys: Vec<_> = map.keys().copied().collect();
        for key in keys {
            let c = map[&key];
            if c == 0 {
                continue;
            }
            match map.get_mut(&(key * 2)) {
                Some(v) => {
                    if *v < c {
                        return false;
                    }
                    *v -= c;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut postives = BTreeMap::new();
        let mut positive_cnt = 0;
        let mut negatives = BTreeMap::new();
        let mut negative_cnt = 0;
        let mut zero_cnt = 0;
        for i in arr {
            if i > 0 {
                *postives.entry(i).or_insert(0) += 1;
                positive_cnt += 1;
            } else if i == 0 {
                zero_cnt += 1;
            } else if i < 0 {
                *negatives.entry(-i).or_insert(0) += 1;
                negative_cnt += 1;
            }
        }
        if zero_cnt % 2 == 1 || positive_cnt % 2 == 1 || negative_cnt % 2 == 1 {
            return false;
        }
        Self::check_map(&mut postives) && Self::check_map(&mut negatives)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_reorder_doubled() {
        assert_eq!(Solution::can_reorder_doubled(vec![3, 1, 3, 6]), false);
        assert_eq!(Solution::can_reorder_doubled(vec![2, 1, 2, 6]), false);
        assert_eq!(Solution::can_reorder_doubled(vec![4, -2, 2, -4]), true);
    }
}
