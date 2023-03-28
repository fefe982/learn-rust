// https://leetcode.com/problems/minimum-cost-for-tickets/
// 983. Minimum Cost For Tickets
pub struct Solution;
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut cost_day = vec![0; days.len() + 1];
        let mut p7 = 0;
        let mut p30 = 0;
        for idx in 0..days.len() {
            cost_day[idx + 1] = std::cmp::min(
                std::cmp::min(cost_day[idx] + costs[0], cost_day[p7] + costs[1]),
                cost_day[p30] + costs[2],
            );
            if idx + 1 < days.len() {
                while days[p7] + 7 <= days[idx + 1] {
                    p7 += 1;
                }
                while days[p30] + 30 <= days[idx + 1] {
                    p30 += 1;
                }
            }
        }
        cost_day[days.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mincost_tickets() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
            11
        );
        assert_eq!(
            Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
            17
        );
    }
}
