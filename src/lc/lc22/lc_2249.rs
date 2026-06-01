// https://leetcode.com/problems/count-lattice-points-inside-a-circle/
// 2249. Count Lattice Points Inside a Circle
pub struct Solution;
impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let mut circles = circles;
        circles.sort_unstable_by_key(|c| -c[2]);
        let mut minx = i32::MAX;
        let mut maxx = i32::MIN;
        let mut miny = i32::MAX;
        let mut maxy = i32::MIN;
        for circle in &circles {
            let (x, y, r) = (circle[0], circle[1], circle[2]);
            minx = minx.min(x - r);
            maxx = maxx.max(x + r);
            miny = miny.min(y - r);
            maxy = maxy.max(y + r);
        }
        let mut cnt=0;
        for x in minx..=maxx {
            for y in miny..=maxy {
                for circle in &circles {
                    let (cx, cy, r) = (circle[0], circle[1], circle[2]);
                    if (x - cx) * (x - cx) + (y - cy) * (y - cy) <= r * r {
                        cnt += 1;
                        break;
                    }
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_lattice_points() {
        assert_eq!(
            Solution::count_lattice_points(vec_vec![[2, 2, 1]]),
            5
        );
        assert_eq!(
            Solution::count_lattice_points(vec_vec![[2, 2, 2], [3, 4, 1]]),
            16
        );
    }
}