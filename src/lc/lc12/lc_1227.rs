// https://leetcode.com/problems/airplane-seat-assignment-probability/
// 1227. Airplane Seat Assignment Probability
pub struct Solution;
impl Solution {
    pub fn nth_person_gets_nth_seat(n: i32) -> f64 {
        if n == 1 {
            1.0
        } else {
            0.5
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    #[test]
    fn nth_person_gets_nth_seat() {
        assert_approx_eq!(Solution::nth_person_gets_nth_seat(1), 1.00000);
        assert_approx_eq!(Solution::nth_person_gets_nth_seat(2), 0.50000);
    }
}
