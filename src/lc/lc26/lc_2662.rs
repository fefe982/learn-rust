// https://leetcode.com/problems/minimum-cost-of-a-path-with-special-roads/
// 2662. Minimum Cost of a Path with Special Roads
pub struct Solution;
impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
        let mut ans = (target[0] - start[0]).abs() + (target[1] - start[1]).abs();
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), start[0], start[1]));
        let mut d = std::collections::HashSet::new();
        while let Some((std::cmp::Reverse(cost), x, y)) = q.pop() {
            if d.contains(&(x, y)) {
                continue;
            }
            (&mut d).insert((x, y));
            for r in special_roads.iter() {
                let rsx = r[0];
                let rsy = r[1];
                let rtx = r[2];
                let rty = r[3];
                let rc = r[4];
                if d.contains(&(rtx, rty)) {
                    continue;
                }
                let dt = cost + (rsx - x).abs() + (rsy - y).abs() + rc;
                if dt > ans {
                    continue;
                }
                q.push((std::cmp::Reverse(dt), rtx, rty));
                ans = ans.min(dt + (rtx - target[0]).abs() + (rty - target[1]).abs());
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                vec![1, 1],
                vec![7, 9],
                vec_vec![[1, 3, 1, 9, 1], [1, 9, 7, 8, 5], [1, 3, 4, 2, 5], [5, 5, 7, 5, 4]]
            ),
            9
        );
        assert_eq!(
            Solution::minimum_cost(
                vec![1, 1],
                vec![10, 8],
                vec_vec![[6, 4, 9, 7, 1], [5, 2, 2, 1, 3], [3, 2, 5, 5, 2]]
            ),
            10
        );
        assert_eq!(
            Solution::minimum_cost(vec![1, 1], vec![4, 5], vec_vec![[1, 2, 3, 3, 2], [3, 4, 4, 5, 1]]),
            5
        );
        assert_eq!(
            Solution::minimum_cost(vec![1, 1], vec![4, 5], vec_vec![[1, 2, 3, 3, 2], [3, 4, 4, 5, 1]]),
            5
        );
        assert_eq!(
            Solution::minimum_cost(
                vec![3, 2],
                vec![5, 7],
                vec_vec![[5, 7, 3, 2, 1], [3, 2, 3, 4, 4], [3, 3, 5, 5, 5], [3, 4, 5, 6, 6]]
            ),
            7
        );
        assert_eq!(
            Solution::minimum_cost(
                vec![1, 1],
                vec![10, 4],
                vec_vec![[4, 2, 1, 1, 3], [1, 2, 7, 4, 4], [10, 3, 6, 1, 2], [6, 1, 1, 2, 3]]
            ),
            8
        );
    }
}
