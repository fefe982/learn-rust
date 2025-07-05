// https://leetcode.com/problems/minimum-skips-to-arrive-at-meeting-on-time/
// 1883. Minimum Skips to Arrive at Meeting On Time
pub struct Solution;
impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let mut min_d = vec![0; dist.len() + 1];
        for i in 1..=dist.len() {
            min_d[i] = min_d[i - 1] + dist[i - 1];
            for j in (1..i).rev() {
                min_d[j] = ((min_d[j] + speed - 1) / speed * speed + dist[i - 1]).min(min_d[j - 1] + dist[i - 1]);
            }
            min_d[0] = (min_d[0] + speed - 1) / speed * speed + dist[i - 1];
        }
        for i in 0..dist.len() {
            if min_d[i] <= hours_before * speed {
                return i as i32;
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_skips() {
        assert_eq!(Solution::min_skips(vec![1, 3, 2], 4, 2), 1);
        assert_eq!(Solution::min_skips(vec![7, 3, 5, 5], 2, 10), 2);
        assert_eq!(Solution::min_skips(vec![7, 3, 5, 5], 1, 10), -1);
    }
}
