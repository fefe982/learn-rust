// https://leetcode.com/problems/corporate-flight-bookings/
// 1109. Corporate Flight Bookings
pub struct Solution;
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut diff = vec![0; n + 1];
        for booking in bookings {
            let (i, j, k) = (booking[0] as usize, booking[1] as usize, booking[2]);
            diff[i - 1] += k;
            diff[j] -= k;
        }
        let mut ans = vec![0; n];
        ans[0] = diff[0];
        for i in 1..n {
            ans[i] = ans[i - 1] + diff[i];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn corp_flight_bookings() {
        assert_eq!(
            Solution::corp_flight_bookings(vec_vec![[1, 2, 10], [2, 3, 20], [2, 5, 25]], 5),
            vec![10, 55, 45, 25, 25]
        );
        assert_eq!(
            Solution::corp_flight_bookings(vec_vec![[1, 2, 10], [2, 2, 15]], 2),
            vec![10, 25]
        );
    }
}
