// https://leetcode.com/problems/count-fertile-pyramids-in-a-land/
// 2088. Count Fertile Pyramids in a Land
pub struct Solution;
impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() < 2 || grid[0].len() < 3 {
            return 0;
        }
        let mut grid1 = grid.clone();
        let mut sum = 0;
        for i in 1..grid1.len() {
            let mut q = std::collections::BinaryHeap::new();
            q.push((std::cmp::Reverse(grid1[i - 1][0]), 0));
            q.push((std::cmp::Reverse(grid1[i - 1][1]), 1));
            for j in 1..grid1[0].len() - 1 {
                q.push((std::cmp::Reverse(grid1[i - 1][j + 1]), j + 1));
                while let Some((std::cmp::Reverse(v), vi)) = q.peek() {
                    if vi + 1 < j {
                        q.pop();
                    } else {
                        if grid1[i][j] == 1 {
                            grid1[i][j] = v + 1;
                            sum += v;
                        }
                        break;
                    }
                }
            }
        }
        let mut grid1 = grid;
        for i in (0..grid1.len() - 1).rev() {
            let mut q = std::collections::BinaryHeap::new();
            q.push((std::cmp::Reverse(grid1[i + 1][0]), 0));
            q.push((std::cmp::Reverse(grid1[i + 1][1]), 1));
            for j in 1..grid1[0].len() - 1 {
                q.push((std::cmp::Reverse(grid1[i + 1][j + 1]), j + 1));
                while let Some((std::cmp::Reverse(v), vi)) = q.peek() {
                    if vi + 1 < j {
                        q.pop();
                    } else {
                        if grid1[i][j] == 1 {
                            grid1[i][j] = v + 1;
                            sum += v;
                        }
                        break;
                    }
                }
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_pyramids() {
        assert_eq!(Solution::count_pyramids(vec_vec![[0, 1, 1, 0], [1, 1, 1, 1]]), 2);
        assert_eq!(Solution::count_pyramids(vec_vec![[1, 1, 1], [1, 1, 1]]), 2);
        assert_eq!(
            Solution::count_pyramids(vec_vec![
                [1, 1, 1, 1, 0],
                [1, 1, 1, 1, 1],
                [1, 1, 1, 1, 1],
                [0, 1, 0, 0, 1]
            ]),
            13
        );
    }
}
