// https://leetcode.com/problems/minimum-cost-to-set-cooking-time/
// 2162. Minimum Cost to Set Cooking Time
pub struct Solution;
impl Solution {
    pub fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32 {
        let cost = |start_at: i32, move_cost: i32, push_cost: i32, minute: i32, second: i32| {
            let mut res = 0;
            let mut prev = start_at;
            if minute != 0 {
                if minute / 10 != 0 {
                    if minute / 10 != prev {
                        res += move_cost;
                    }
                    res += push_cost;
                    prev = minute / 10;
                }
                if minute % 10 != prev {
                    res += move_cost;
                }
                res += push_cost;
                prev = minute % 10;
            }
            if (minute == 0 && second >= 10) || minute != 0 {
                if second / 10 != prev {
                    res += move_cost;
                }
                res += push_cost;
                prev = second / 10;
            }
            if second % 10 != prev {
                res += move_cost;
            }
            res += push_cost;
            res
        };
        let mut minute = target_seconds / 60;
        let mut second = target_seconds % 60;
        let mut res = i32::MAX;
        if minute < 100 {
            res = cost(start_at, move_cost, push_cost, minute, second);
        }
        if minute >= 1 && second <= 39 {
            minute -= 1;
            second += 60;
            res = res.min(cost(start_at, move_cost, push_cost, minute, second));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_cost_set_time() {
        assert_eq!(Solution::min_cost_set_time(7, 220, 479, 6000), 2576);
        assert_eq!(Solution::min_cost_set_time(1, 2, 1, 600), 6);
        assert_eq!(Solution::min_cost_set_time(0, 1, 2, 76), 6);
    }
}
