// https://leetcode.com/problems/maximum-walls-destroyed-by-robots/
// 3661. Maximum Walls Destroyed by Robots
pub struct Solution;
impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let mut robots = robots.into_iter().zip(distance.into_iter()).collect::<Vec<_>>();
        robots.push((i32::MIN, 0));
        robots.push((i32::MAX, 0));
        robots.sort();
        let mut walls = walls;
        walls.sort();
        let mut cl = 0;
        let mut cr = 0;
        let mut iw = 0;
        let mut cclr = 0;
        for ib in 1..robots.len() - 1 {
            let (pb, db) = robots[ib];
            let lb = pb - db;
            let (npb, ndb) = robots[ib + 1];
            let rb = (pb + db).min(npb - 1);
            let nlb = npb - ndb;
            let rbb = rb.min(nlb - 1);
            let mut ccl = 0;
            let mut ccr = 0;
            while iw < walls.len() && walls[iw] < lb {
                iw += 1;
            }
            while iw < walls.len() && walls[iw] < pb {
                ccl += 1;
                iw += 1;
            }
            if iw < walls.len() && walls[iw] == pb {
                ccl += 1;
                ccr += 1;
                iw += 1;
            }
            let ncl = (cl + ccl + cclr).max(cr + ccl);
            cclr = 0;
            while iw < walls.len() && walls[iw] <= rbb {
                ccr += 1;
                iw += 1;
            }
            while iw < walls.len() && walls[iw] <= rb {
                ccr += 1;
                cclr += 1;
                iw += 1;
            }
            cr = cl.max(cr) + ccr;
            cl = ncl;
        }
        cl.max(cr)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_walls() {
        assert_eq!(
            Solution::max_walls(
                vec![17, 59, 32, 11, 72, 18],
                vec![5, 7, 6, 5, 2, 10],
                vec![
                    17, 25, 33, 29, 54, 53, 18, 35, 39, 37, 20, 14, 34, 13, 16, 58, 22, 51, 56, 27, 10, 15, 12, 23, 45,
                    43, 21, 2, 42, 7, 32, 40, 8, 9, 1, 5, 55, 30, 38, 4, 3, 31, 36, 41, 57, 28, 11, 49, 26, 19, 50, 52,
                    6, 47, 46, 44, 24, 48
                ]
            ),
            37
        );
        assert_eq!(Solution::max_walls(vec![4], vec![3], vec![1, 10]), 1);
        assert_eq!(Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]), 3);
        assert_eq!(Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]), 0);
    }
}
