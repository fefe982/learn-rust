// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/
// 1353. Maximum Number of Events That Can Be Attended
pub struct Solution;
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_unstable();
        let mut q = std::collections::BinaryHeap::new();
        let mut day = -1;
        let mut ans = 0;
        let mut i = 0;
        while i < events.len() || !q.is_empty() {
            if q.is_empty() {
                day = events[i][0];
            }
            while i < events.len() && events[i][0] <= day {
                q.push(std::cmp::Reverse(events[i][1]));
                i += 1;
            }
            while let Some(e) = q.pop() {
                if e.0 < day {
                    continue;
                }
                ans += 1;
                day += 1;
                break;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_events() {
        assert_eq!(
            Solution::max_events(vec_vec![[1, 5], [1, 5], [1, 5], [2, 3], [2, 3]]),
            5
        );
        assert_eq!(
            Solution::max_events(vec_vec![[1, 2], [1, 2], [3, 3], [1, 5], [1, 5]]),
            5
        );
        assert_eq!(Solution::max_events(vec_vec![[1, 2], [2, 3], [3, 4]]), 3);
        assert_eq!(Solution::max_events(vec_vec![[1, 2], [2, 3], [3, 4], [1, 2]]), 4);
    }
}
