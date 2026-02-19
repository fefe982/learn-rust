// https://leetcode.com/problems/maximize-the-distance-between-points-on-a-square/solutions/
// 3464. Maximize the Distance Between Points on a Square
pub struct Solution;
impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let side = side as i64;
        let mut p = points
            .into_iter()
            .map(|p| {
                if p[0] == 0 {
                    p[1] as i64
                } else if p[1] as i64 == side {
                    side + p[0] as i64
                } else if p[0] as i64 == side {
                    side * 3 - p[1] as i64
                } else {
                    side * 4 - p[0] as i64
                }
            })
            .collect::<Vec<_>>();
        p.sort();
        let check = |low: i64| {
            for &start in p.iter() {
                let end = start + side * 4 - low;
                let mut failed = false;
                let mut s = start;
                for _ in 1..k {
                    let ip = p.partition_point(|&pp| (pp - s) < low);
                    if ip == p.len() || p[ip] > end {
                        failed = true;
                        break;
                    }
                    s = p[ip];
                }
                if !failed {
                    return true;
                }
            }
            false
        };
        let mut left = 1;
        let mut right = side * 4 / k as i64 + 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }
        left as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_distance() {
        assert_eq!(
            Solution::max_distance(2, vec_vec![[0, 2], [2, 0], [2, 2], [0, 0]], 4),
            2
        );
        assert_eq!(
            Solution::max_distance(2, vec_vec![[0, 0], [1, 2], [2, 0], [2, 2], [2, 1]], 4),
            1
        );
        assert_eq!(
            Solution::max_distance(2, vec_vec![[0, 0], [0, 1], [0, 2], [1, 2], [2, 0], [2, 2], [2, 1]], 5),
            1
        );
    }
}
