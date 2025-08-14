// https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/
// 1011. Capacity To Ship Packages Within D Days
use std::cmp;
pub struct Solution;
impl Solution {
    fn tray_ship_within_days(weights: &Vec<i32>, ship_weight: i32, days: i32) -> (i32, i32) {
        let mut next_ship_weight = 0;
        let mut this_ship_weight = 0;
        let mut day_count = 0;
        let mut shipped = 0;
        for w in weights.iter() {
            let partial = this_ship_weight + w;
            if partial > ship_weight {
                if next_ship_weight == 0 || partial < next_ship_weight {
                    next_ship_weight = partial;
                }
                day_count += 1;
                if day_count >= days {
                    return (days + 1, next_ship_weight);
                }
                this_ship_weight = *w;
            } else {
                this_ship_weight = partial;
            }
            shipped += w;
        }
        return (day_count + 1, next_ship_weight);
    }

    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut max_weight = 0;
        let mut sum = 0;
        for w in weights.iter() {
            if max_weight < *w {
                max_weight = *w;
            }
            sum += w;
        }
        let mut avg = sum / days;
        if sum % days > 0 {
            avg += 1;
        }
        let mut res = cmp::max(avg, max_weight);
        loop {
            let (expected_days, new_res) = Self::tray_ship_within_days(&weights, res, days);
            if expected_days <= days {
                return res;
            }
            res = new_res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ship_within_days() {
        assert_eq!(
            Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
            15
        );
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
        assert_eq!(
            Solution::ship_within_days(
                vec![
                    68, 25, 170, 494, 202, 88, 151, 296, 329, 365, 417, 81, 441, 366, 230, 408,
                    240, 356, 253, 489, 137, 209
                ],
                11
            ),
            696
        )
    }
}
