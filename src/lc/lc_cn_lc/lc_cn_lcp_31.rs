// https://leetcode.cn/problems/Db3wC1/
// LCP 31. 变换的迷宫
pub struct Solution;
impl Solution {
    fn escape(
        maze: &Vec<Vec<Vec<char>>>,
        t: usize,
        x: usize,
        y: usize,
        ma: usize,
        mb: usize,
        cache: &mut Vec<Vec<Vec<[[i32; 2]; 2]>>>,
    ) -> bool {
        let tt = maze.len();
        let xx = maze[0].len();
        let yy = maze[0][0].len();
        let fv = 1;
        let tv = 2;
        if x == xx - 1 && y == yy - 1 {
            return true;
        }
        if t == tt - 1 {
            return false;
        }
        if xx + yy - x - y - 2 > tt - t - 1 {
            return false;
        }
        if cache[t][x][y][ma][mb] == fv {
            return false;
        }
        if cache[t][x][y][ma][mb] == tv {
            return true;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)] {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if nx >= xx || ny >= yy {
                continue;
            }
            if maze[t + 1][nx][ny] == '.' {
                if Self::escape(maze, t + 1, nx, ny, ma, mb, cache) {
                    cache[t][x][y][ma][mb] = tv;
                    return true;
                }
            } else {
                if ma == 0 {
                    if Self::escape(maze, t + 1, nx, ny, 1, mb, cache) {
                        cache[t][x][y][ma][mb] = tv;
                        return true;
                    }
                }
                if mb == 0 {
                    for it in t + 1..tt {
                        if Self::escape(maze, it, nx, ny, ma, 1, cache) {
                            cache[t][x][y][ma][mb] = tv;
                            return true;
                        }
                    }
                }
            }
        }
        cache[t][x][y][ma][mb] = fv;
        false
    }
    pub fn escape_maze(maze: Vec<Vec<String>>) -> bool {
        let maze = maze
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self::escape(
            &maze,
            0,
            0,
            0,
            0,
            0,
            &mut vec![vec![vec![[[0; 2]; 2]; maze[0][0].len()]; maze[0].len()]; maze.len()],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::escape_maze(vec_vec_str![
                [".#.", "#.."],
                ["...", ".#."],
                [".##", ".#."],
                ["..#", ".#."]
            ]),
            true
        );
        assert_eq!(
            Solution::escape_maze(vec_vec_str![[".#.", "..."], ["...", "..."]]),
            false
        );
        assert_eq!(
            Solution::escape_maze(vec_vec_str![
                ["...", "...", "..."],
                [".##", "###", "##."],
                [".##", "###", "##."],
                [".##", "###", "##."],
                [".##", "###", "##."],
                [".##", "###", "##."],
                [".##", "###", "##."]
            ]),
            false
        );
    }
}
