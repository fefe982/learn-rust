// https://leetcode.com/problems/cherry-pickup-ii/
// 1463. Cherry Pickup II
pub struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let mut s = std::collections::HashMap::new();
        let m = grid.len();
        let n = grid[0].len();
        s.insert((0, grid[0].len() - 1), grid[0][0] + grid[0][grid[0].len() - 1]);
        for i in 1..m {
            let mut t = std::collections::HashMap::<(usize, usize), i32>::new();
            for (&(x1, x2), v1) in s.iter() {
                for xx1 in (x1.max(1)) - 1..=(x1 + 1).min(n - 1) {
                    for xx2 in x2.max(xx1 + 1) - 1..=(x2 + 1).min(n - 1) {
                        let mut v = v1 + grid[i][xx1];
                        if xx2 != xx1 {
                            v += grid[i][xx2];
                        }
                        let ev = t.entry((xx1, xx2)).or_default();
                        *ev = (*ev).max(v);
                    }
                }
            }
            s = t;
        }
        *s.values().max().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cherry_pickup() {
        assert_eq!(
            Solution::cherry_pickup(vec_vec![[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]]),
            24
        );
        assert_eq!(
            Solution::cherry_pickup(vec_vec![
                [1, 0, 0, 0, 0, 0, 1],
                [2, 0, 0, 0, 0, 3, 0],
                [2, 0, 9, 0, 0, 0, 0],
                [0, 3, 0, 5, 4, 0, 0],
                [1, 0, 2, 3, 0, 0, 6]
            ]),
            28
        );
    }
}
