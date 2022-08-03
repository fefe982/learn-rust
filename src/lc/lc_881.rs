// https://leetcode.com/problems/boats-to-save-people/
// 881. Boats to Save People
use std::collections::BTreeMap;
pub struct Solution;
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut low = 40000;
        let mut high = 0;
        let mut cnt = BTreeMap::<i32, i32>::new();
        for p in people {
            *cnt.entry(p).or_default() += 1;
            low = std::cmp::min(low, p);
            high = std::cmp::max(high, p);
        }
        let mut num = 0;
        let keys: Vec<_> = cnt.keys().cloned().collect();
        let mut low_idx = 0;
        let mut high_idx = keys.len() - 1;
        if keys[high_idx] == limit {
            num += cnt[&limit];
            if high_idx == 0 {
                return num;
            }
            high_idx -= 1;
        }
        while high_idx > low_idx {
            let key_high = keys[high_idx];
            let key_low = keys[low_idx];
            if key_high + key_low > limit {
                num += cnt[&key_high];
                high_idx -= 1;
            } else {
                let val_high = cnt[&key_high];
                let val_low = cnt[&key_low];
                if val_high == val_low {
                    num += val_high;
                    high_idx -= 1;
                    low_idx += 1;
                } else if val_high > val_low {
                    num += val_low;
                    low_idx += 1;
                    *cnt.get_mut(&key_high).unwrap() -= val_low;
                } else {
                    num += val_high;
                    high_idx -= 1;
                    *cnt.get_mut(&key_low).unwrap() -= val_high;
                }
            }
        }
        if high_idx == low_idx {
            let key = keys[high_idx];
            let val = cnt[&key];
            if key * 2 <= limit {
                num += (val + 1) / 2
            } else {
                num += val
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_rescue_boats() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }
}
