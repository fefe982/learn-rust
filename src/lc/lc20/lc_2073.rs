// https://leetcode.com/problems/time-needed-to-buy-tickets/
// 2073. Time Needed to Buy Tickets
pub struct Solution;
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let sum = (tickets[k as usize] - 1) * tickets.len() as i32 + k + 1;
        let n = tickets[k as usize];
        tickets.into_iter().enumerate().fold(sum, |acc, (i, x)| {
            if x >= n {
                acc
            } else {
                if i < k as usize {
                    acc - (n - x)
                } else {
                    acc - (n - x - 1)
                }
            }
        })
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
