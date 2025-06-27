// https://leetcode.com/problems/maximize-distance-to-closest-person/
// 849. Maximize Distance to Closest Person
pub struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut i = 0;
        while seats[i] != 1 {
            i += 1;
        }
        let mut max = i;
        let mut last = i;
        while i < seats.len() {
            i += 1;
            while i < seats.len() && seats[i] != 1 {
                i += 1;
            }
            if i == seats.len() {
                max = max.max(i - last - 1);
                break;
            } else {
                max = max.max((i - last) / 2);
                last = i;
            }
        }
        max as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_dist_to_closest() {
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]), 2);
        assert_eq!(Solution::max_dist_to_closest(vec![1, 0, 0, 0]), 3);
        assert_eq!(Solution::max_dist_to_closest(vec![0, 1]), 1);
    }
}
