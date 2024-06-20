// https://leetcode.com/problems/magnetic-force-between-two-balls/
// 1552. Magnetic Force Between Two Balls
pub struct Solution;
impl Solution {
    fn check(position: &Vec<i32>, m: i32, dist: i32) -> bool {
        let mut ip = 0;
        for _ in 1..m {
            let np = position[ip] + dist;
            let nip = position[ip..].partition_point(|x| *x < np) + ip;
            if nip == position.len() {
                return false;
            }
            ip = nip;
        }
        true
    }
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut position = position;
        position.sort_unstable();
        let mut high = (position.last().unwrap() - position.first().unwrap()) / (m - 1);
        if Self::check(&position, m, high) {
            return high;
        }
        let mut low = 1;
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Self::check(&position, m, mid) {
                low = mid;
            } else {
                high = mid;
            }
        }
        low
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_distance() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
        assert_eq!(Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2), 999999999);
    }
}
