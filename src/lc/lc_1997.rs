// https://leetcode.com/problems/first-day-where-you-have-been-in-all-the-rooms/
// 1997. First Day Where You Have Been in All the Rooms
pub struct Solution;
impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let m = 1_0000_0000_7i64;
        let mut dp = vec![0; next_visit.len() - 1];
        let mut sum = vec![0; next_visit.len()];
        for i in 0..next_visit.len() - 1 {
            dp[i] = (sum[i] - sum[next_visit[i] as usize] + m + 2) % m;
            sum[i + 1] = (sum[i] + dp[i]) % m;
        }
        sum[next_visit.len() - 1] as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_day_been_in_all_rooms() {
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0]), 2);
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 0, 2]), 6);
        assert_eq!(Solution::first_day_been_in_all_rooms(vec![0, 1, 2, 0]), 6);
    }
}
