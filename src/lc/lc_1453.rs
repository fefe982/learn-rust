// https://leetcode.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard/
// 1453. Maximum Number of Darts Inside of a Circular Dartboard
pub struct Solution;
impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let mut ans = 1;
        let r2 = r * r;
        for d in &darts {
            let x = d[0];
            let y = d[1];
            let mut angles = vec![];
            for dd in &darts {
                let xx = dd[0];
                let yy = dd[1];
                if xx == x && yy == y {
                    continue;
                }
                let dd = (x - xx) * (x - xx) + (y - yy) * (y - yy);
                if dd > 4 * r2 {
                    continue;
                }
                let dir = ((yy - y) as f64).atan2((xx - x) as f64);
                let delta = ((dd as f64).sqrt() / (2.0 * r as f64)).acos();
                angles.push((((dir - delta) * 1_000000.0) as i32, -1));
                angles.push((((dir + delta) * 1_000000.0) as i32, 1));
            }
            angles.sort();
            angles.into_iter().fold(1, |cnt, (_, d)| {
                let ncnt = cnt - d;
                ans = ans.max(ncnt);
                ncnt
            });
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_num_points() {
        assert_eq!(Solution::num_points(vec_vec![[-2, 0], [2, 0], [0, 2], [0, -2]], 2), 4);
        assert_eq!(
            Solution::num_points(vec_vec![[-3, 0], [3, 0], [2, 6], [5, 4], [0, 9], [7, 8]], 5),
            5
        );
    }
}
