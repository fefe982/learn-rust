// https://leetcode.com/problems/ipo/
// 502. IPO
pub struct Solution;
use std::collections::BTreeMap;
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut capital_profit_map: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();
        for (prof, cap) in profits.iter().zip(capital.iter()) {
            if let Some(v) = capital_profit_map.get_mut(cap) {
                if let Some(cnt) = v.get_mut(prof) {
                    *cnt += 1;
                } else {
                    v.insert(*prof, 1);
                }
            } else {
                let mut prof_map = BTreeMap::new();
                prof_map.insert(*prof, 1);
                capital_profit_map.insert(*cap, prof_map);
            }
        }
        let mut capital_max_profit: BTreeMap<i32, (i32, i32)> = BTreeMap::new();
        let mut last_max = -1;
        let mut last_cap: i32 = -1;
        for (cap, prof_map) in capital_profit_map.iter() {
            if let Some((this_max_profit, _)) = prof_map.iter().rev().next() {
                if *this_max_profit > last_max {
                    last_max = *this_max_profit;
                    last_cap = *cap;
                }
            }
            capital_max_profit.insert(*cap, (last_max, last_cap));
        }
        let mut current = w;
        for _ in 0..k {
            let mut cur_profit = 0;
            let mut cur_cap = 0;
            if let Some((_, (cur_profit_, cur_cap_))) =
                capital_max_profit.range(..=current).next_back()
            {
                cur_profit = *cur_profit_;
                cur_cap = *cur_cap_;
            }
            let mut last_max_profit = -1;
            let mut last_max_profit_cap = -1;
            if let Some((_, (last_profit, last_profit_idx))) =
                capital_max_profit.range(..cur_cap).next_back()
            {
                last_max_profit = *last_profit;
                last_max_profit_cap = *last_profit_idx;
            }
            let mut removed = false;
            if let Some(prof_map) = capital_profit_map.get_mut(&cur_cap) {
                if let Some(cnt) = prof_map.get_mut(&cur_profit) {
                    *cnt -= 1;
                    if *cnt <= 0 {
                        removed = true;
                        prof_map.remove(&cur_profit);
                    }
                }
                if prof_map.is_empty() {
                    capital_profit_map.remove(&cur_cap);
                    capital_max_profit.remove(&cur_cap);
                }
            }
            if removed {
                for (cap, (max_prof, prof_cap)) in capital_max_profit.range_mut(cur_cap..) {
                    if *max_prof > cur_profit {
                        break;
                    }
                    if let Some(prof_map) = capital_profit_map.get(cap) {
                        if let Some((cap_map_prof, _)) = prof_map.iter().rev().next() {
                            if *cap_map_prof >= last_max_profit {
                                *max_prof = *cap_map_prof;
                                *prof_cap = *cap;
                                last_max_profit = *cap_map_prof;
                                last_max_profit_cap = *cap;
                            } else {
                                *max_prof = last_max_profit;
                                *prof_cap = last_max_profit_cap;
                            }
                        }
                    }
                }
            }
            current += cur_profit;
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ship_within_days() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }
}
