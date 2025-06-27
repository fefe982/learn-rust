// https://leetcode.com/problems/walking-robot-simulation/
// 874. Walking Robot Simulation
pub struct Solution;
impl Solution {
    fn checkob(pos: i32, oob: Option<&std::collections::BTreeSet<i32>>, dir: i32) -> i32 {
        if let Some(ob) = oob {
            if dir > 0 {
                if let Some(&n) = ob.range(pos + 1..).next() {
                    n - pos - 1
                } else {
                    i32::MAX
                }
            } else if let Some(&n) = ob.range(..pos).rev().next() {
                pos - n - 1
            } else {
                i32::MAX
            }
        } else {
            i32::MAX
        }
    }
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut dir = (0, 1);
        let mut max = 0;
        let mut pos = (0, 0);
        let mut obx = std::collections::HashMap::<i32, std::collections::BTreeSet<i32>>::new();
        let mut oby = obx.clone();
        for ob in obstacles {
            obx.entry(ob[0]).or_default().insert(ob[1]);
            oby.entry(ob[1]).or_default().insert(ob[0]);
        }
        for comm in commands {
            match comm {
                -1 => dir = (dir.1, -dir.0),
                -2 => dir = (-dir.1, dir.0),
                1..=9 => {
                    let dst = comm.min(if dir.0 != 0 {
                        Self::checkob(pos.0, oby.get(&pos.1), dir.0)
                    } else {
                        Self::checkob(pos.1, obx.get(&pos.0), dir.1)
                    });
                    pos = (pos.0 + dir.0 * dst, pos.1 + dir.1 * dst);
                    max = max.max(pos.0 * pos.0 + pos.1 * pos.1);
                }
                _ => unreachable!(),
            }
        }
        max
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn robot_sim() {
        assert_eq!(
            Solution::robot_sim(
                vec![2, 2, 5, -1, -1],
                vec_vec![
                    [-3, 5],
                    [-2, 5],
                    [3, 2],
                    [5, 0],
                    [-2, 0],
                    [-1, 5],
                    [5, -3],
                    [0, 0],
                    [-4, 4],
                    [-3, 4]
                ]
            ),
            81
        );
        assert_eq!(Solution::robot_sim(vec![4, -1, 3], vec![]), 25);
        assert_eq!(
            Solution::robot_sim(vec![4, -1, 4, -2, 4], vec_vec![[2, 4]]),
            65
        );
        assert_eq!(Solution::robot_sim(vec![6, -1, -1, 6], vec![]), 36);
    }
}
