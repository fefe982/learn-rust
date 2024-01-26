// https://leetcode.com/problems/out-of-boundary-paths/
// 576. Out of Boundary Paths
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
#[derive(Debug, Clone, Copy)]
struct IMod {
    val: i64,
}
impl IMod {
    fn new(val: i64) -> Self {
        Self { val }
    }
    fn as_i32(&self) -> i32 {
        self.val as i32
    }
}
impl std::ops::AddAssign for IMod {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % MOD;
    }
}
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut ans = IMod::new(0);
        let mut board = vec![vec![ans; n as usize]; m as usize];
        board[start_row as usize][start_column as usize] = IMod::new(1);
        for _ in 0..max_move {
            let mut next = vec![vec![IMod::new(0); n as usize]; m as usize];
            for i in 0..m {
                for j in 0..n {
                    for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                        let nx = i + dx;
                        let ny = j + dy;
                        if nx < 0 || ny < 0 || nx >= m || ny >= n {
                            ans += board[i as usize][j as usize];
                        } else {
                            next[nx as usize][ny as usize] += board[i as usize][j as usize];
                        }
                    }
                }
            }
            board = next;
        }
        ans.as_i32()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_paths() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }
}
