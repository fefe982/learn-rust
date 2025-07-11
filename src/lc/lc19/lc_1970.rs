// https://leetcode.com/problems/last-day-where-you-can-still-cross/
// 1970. Last Day Where You Can Still Cross
pub struct Solution;
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let mut field = vec![vec![0; col]; row];
        for (cnt, cell) in cells.into_iter().enumerate() {
            let ir = cell[0] as usize - 1;
            let ic = cell[1] as usize - 1;
            let mut q = std::collections::VecDeque::new();
            field[ir][ic] = 1;
            if ic == 0 {
                field[ir][ic] = 2;
                q.push_back((ir, ic));
            } else {
                'lbl: for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let nr = (ir as i32 + i) as usize;
                        let nc = (ic as i32 + j) as usize;
                        if nr >= row || nc >= col {
                            continue;
                        }
                        if field[nr][nc] == 2 {
                            if ic == col - 1 {
                                return cnt as i32;
                            }
                            field[ir][ic] = 2;
                            q.push_back((ir, ic));
                            break 'lbl;
                        }
                    }
                }
            }
            while let Some((nr, nc)) = q.pop_front() {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        let nnr = (nr as i32 + i) as usize;
                        let nnc = (nc as i32 + j) as usize;
                        if nnr >= row || nnc >= col {
                            continue;
                        }
                        if field[nnr][nnc] == 1 {
                            field[nnr][nnc] = 2;
                            q.push_back((nnr, nnc));
                            if nnc == col - 1 {
                                return cnt as i32;
                            }
                        }
                    }
                }
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn latest_day_to_cross() {
        assert_eq!(
            Solution::latest_day_to_cross(2, 2, vec_vec![[1, 1], [2, 1], [1, 2], [2, 2]]),
            2
        );
        assert_eq!(
            Solution::latest_day_to_cross(2, 2, vec_vec![[1, 1], [1, 2], [2, 1], [2, 2]]),
            1
        );
        assert_eq!(
            Solution::latest_day_to_cross(
                3,
                3,
                vec_vec![
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
            ),
            3
        );
    }
}
