// https://leetcode.com/problems/cat-and-mouse-ii/
// 1728. Cat and Mouse II
pub struct Solution;
impl Solution {
    fn dfs(
        step: usize,
        mpos: (usize, usize),
        cpos: (usize, usize),
        grid: &Vec<Vec<char>>,
        mouse_jump: i32,
        cat_jump: i32,
        nfloor: usize,
        memory: &mut std::collections::HashMap<(usize, (usize, usize), (usize, usize)), bool>,
    ) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        if step > nfloor * 2 {
            return false;
        }
        if let Some(&v) = memory.get(&(step, mpos, cpos)) {
            return v;
        }
        let res = || -> bool {
            if step % 2 == 0 {
                // mouse turn
                for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                    for mv in 1..=mouse_jump {
                        let (i, j) = ((mpos.0 as i32 + di * mv) as usize, (mpos.1 as i32 + dj * mv) as usize);
                        if i < m && j < n {
                            if (i, j) == cpos {
                                continue;
                            }
                            if grid[i][j] == '.' {
                                if Self::dfs(step + 1, (i, j), cpos, grid, mouse_jump, cat_jump, nfloor, memory) {
                                    return true;
                                }
                            } else if grid[i][j] == '#' {
                                break;
                            } else if grid[i][j] == 'F' {
                                return true;
                            }
                        }
                    }
                }
            } else {
                // cat turn
                for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                    for cv in 1..=cat_jump {
                        let (i, j) = ((cpos.0 as i32 + di * cv) as usize, (cpos.1 as i32 + dj * cv) as usize);
                        if i < m && j < n {
                            if (i, j) == mpos {
                                return false;
                            }
                            if grid[i][j] == '.' {
                                if !Self::dfs(step + 1, mpos, (i, j), grid, mouse_jump, cat_jump, nfloor, memory) {
                                    return false;
                                }
                            } else if grid[i][j] == '#' {
                                break;
                            } else if grid[i][j] == 'F' {
                                return false;
                            }
                        }
                    }
                }
            }
            Self::dfs(step + 1, mpos, cpos, grid, mouse_jump, cat_jump, nfloor, memory)
        }();
        memory.insert((step, mpos, cpos), res);
        res
    }
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let mut mpos = (0, 0);
        let mut cpos = (0, 0);
        let (m, n) = (grid.len(), grid[0].len());
        let mut nfloor = m * n;
        let grid = grid
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                row.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'M' => {
                            mpos = (i, j);
                            '.'
                        }
                        'C' => {
                            cpos = (i, j);
                            '.'
                        }
                        '#' => {
                            nfloor -= 1;
                            '#'
                        }
                        _ => c,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut memory = std::collections::HashMap::<(usize, (usize, usize), (usize, usize)), bool>::new();
        Self::dfs(0, mpos, cpos, &grid, mouse_jump, cat_jump, nfloor, &mut memory)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_can_mouse_win() {
        assert_eq!(
            Solution::can_mouse_win(
                vec_str!["#..C...", "M....#.", "######.", "######.", "######.", "######F"],
                1,
                5
            ),
            false
        );
        assert_eq!(Solution::can_mouse_win(vec_str!["####F", "#C...", "M...."], 1, 2), true);
        assert_eq!(
            Solution::can_mouse_win(vec_str!["C...#", "...#F", "....#", "M...."], 2, 5),
            false
        );
        assert_eq!(Solution::can_mouse_win(vec_str!["M.C...F"], 1, 4), true);
        assert_eq!(Solution::can_mouse_win(vec_str!["M.C...F"], 1, 3), false);
    }
}
