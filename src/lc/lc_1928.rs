// https://leetcode.com/problems/minimum-cost-to-reach-destination-in-time/
// 1928. Minimum Cost to Reach Destination in Time
pub struct Solution;
impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let mut g = vec![Vec::<(usize, i32, i32)>::new(); passing_fees.len()];
        for e in edges {
            g[e[0] as usize].push((e[1] as usize, e[2], passing_fees[e[1] as usize]));
            g[e[1] as usize].push((e[0] as usize, e[2], passing_fees[e[0] as usize]));
        }
        let mut cost = vec![i32::MAX; passing_fees.len()];
        let mut time = vec![i32::MAX; passing_fees.len()];
        cost[0] = passing_fees[0];
        time[0] = 0;
        let mut q = std::collections::BinaryHeap::new();
        q.push(std::cmp::Reverse((passing_fees[0], 0, 0)));
        while let Some(std::cmp::Reverse((c, t, v))) = q.pop() {
            for &(nv, et, ec) in &g[v] {
                if t + et > max_time {
                    continue;
                }
                if c + ec < cost[nv] {
                    cost[nv] = c + ec;
                    time[nv] = t + et;
                    q.push(std::cmp::Reverse((cost[nv], time[nv], nv)));
                } else if t + et < time[nv] {
                    time[nv] = t + et;
                    q.push(std::cmp::Reverse((c + ec, time[nv], nv)));
                }
            }
        }
        if cost[passing_fees.len() - 1] == i32::MAX {
            -1
        } else {
            cost[passing_fees.len() - 1]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_cost() {
        assert_eq!(
            Solution::min_cost(
                30,
                vec_vec![[0, 1, 10], [1, 2, 10], [2, 5, 10], [0, 3, 1], [3, 4, 10], [4, 5, 15]],
                vec![5, 1, 2, 20, 20, 3]
            ),
            11
        );
        assert_eq!(
            Solution::min_cost(
                29,
                vec_vec![[0, 1, 10], [1, 2, 10], [2, 5, 10], [0, 3, 1], [3, 4, 10], [4, 5, 15]],
                vec![5, 1, 2, 20, 20, 3]
            ),
            48
        );
        assert_eq!(
            Solution::min_cost(
                25,
                vec_vec![[0, 1, 10], [1, 2, 10], [2, 5, 10], [0, 3, 1], [3, 4, 10], [4, 5, 15]],
                vec![5, 1, 2, 20, 20, 3]
            ),
            -1
        );
    }
}
