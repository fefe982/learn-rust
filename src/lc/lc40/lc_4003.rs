// https://leetcode.com/problems/minimum-cost-path-with-alternating-directions-iii/
// 4003. Minimum Cost Path With Alternating Directions III
pub struct Solution;
impl Solution {
    pub fn min_cost(m: i32, n: i32, penalty: Vec<Vec<i32>>) -> i64 {
        let mut h = std::collections::BinaryHeap::new();
        let m = m as usize;
        let n = n as usize;
        let mut visited = vec![vec![[false, false]; n]; m];
        h.push(std::cmp::Reverse((1, 1, 0, 0)));
        while let Some(std::cmp::Reverse((cost, parity, x, y))) = h.pop() {
            if visited[x][y][parity] {
                continue;
            }
            visited[x][y][parity] = true;
            if x == m - 1 && y == n - 1 {
                return cost;
            }
            if !visited[x][y][1 - parity] {
                h.push(std::cmp::Reverse((cost + penalty[x][y] as i64, 1 - parity, x, y)));
            }
            for (i, (dx, dy)) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().enumerate() {
                let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                if nx < m && ny < n {
                    let ncost = cost
                        + if i % 2 == parity % 2 { penalty[x][y] as i64 } else { 0 }
                        + (nx + 1) as i64 * (ny + 1) as i64;
                    if !visited[nx][ny][1 - parity] {
                        h.push(std::cmp::Reverse((ncost, 1 - parity, nx, ny)));
                    }
                }
            }
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(2, 2, vec_vec![[5, 3], [1, 4]]), 8);
        assert_eq!(Solution::min_cost(2, 2, vec_vec![[0, 7], [3, 2]]), 7);
        assert_eq!(Solution::min_cost(2, 3, vec_vec![[8, 0, 9], [7, 4, 1]]), 12);
    }
}
