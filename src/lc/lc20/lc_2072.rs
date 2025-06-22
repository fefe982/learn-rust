// https://leetcode.com/problems/time-needed-to-buy-tickets/
// 2072. Time Needed to Buy Tickets
pub struct Solution;
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut tk = tickets[k];
        let mut res = tk;
        for i in 0..k {
            res += tickets[i].min(tk);
        }
        tk -= 1;
        for i in k + 1..tickets.len() {
            res += tickets[i].min(tk);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_time_required_to_buy() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
