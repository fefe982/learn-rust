// https://leetcode.cn/problems/rdmXM7/
// LCP 75. 传送卷轴
pub struct Solution;
impl Solution {
    pub fn challenge_of_the_keeper(maze: Vec<String>) -> i32 {
        let mut maze = maze
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (ni, nj) = (maze.len(), maze[0].len());
        let mut d = vec![vec![i32::MAX; nj]; ni];
        let mut t = (usize::MAX, usize::MAX);
        'i: for i in 0..ni {
            for j in 0..nj {
                if maze[i][j] == 'T' {
                    d[i][j] = 0;
                    t = (i, j);
                    break 'i;
                }
            }
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back((0, t));
        let mut s = (usize::MAX, usize::MAX);
        while let Some((k, (i, j))) = q.pop_front() {
            for (x, y) in [(i + 1, j), (i.wrapping_sub(1), j), (i, j + 1), (i, j.wrapping_sub(1))] {
                if x < ni && y < nj && maze[x][y] != '#' && (&d)[x][y] == i32::MAX {
                    d[x][y] = k + 1;
                    q.push_back((k + 1, (x, y)));
                    if maze[x][y] == 'S' {
                        s = (x, y);
                    }
                }
            }
        }
        if s.0 == usize::MAX {
            return -1;
        }
        let mut m = vec![vec![0; nj]; ni];
        for i in 0..ni {
            for j in 0..nj {
                if maze[i][j] != '#' {
                    if maze[ni - 1 - i][j] != '#' {
                        m[i][j] = m[i][j].max(d[ni - 1 - i][j]);
                    }
                    if maze[i][nj - 1 - j] != '#' {
                        m[i][j] = m[i][j].max(d[i][nj - 1 - j]);
                    }
                }
            }
        }
        let mut qq = std::collections::BinaryHeap::new();
        qq.push((std::cmp::Reverse(0), s));
        while let Some((std::cmp::Reverse(k), (i, j))) = qq.pop() {
            if maze[i][j] == '#' {
                continue;
            }
            if k == i32::MAX {
                return -1;
            }
            maze[i][j] = '#';
            for (x, y) in [(i + 1, j), (i.wrapping_sub(1), j), (i, j + 1), (i, j.wrapping_sub(1))] {
                if x < ni && y < nj && maze[x][y] != '#' {
                    if maze[x][y] == 'T' {
                        return k;
                    } else {
                        qq.push((std::cmp::Reverse(k.max(m[x][y])), (x, y)));
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn challenge_of_the_keeper() {
        assert_eq!(
            Solution::challenge_of_the_keeper(vec_str![".....", "##S..", "...#.", "T.#..", "###.."]),
            7
        );
        assert_eq!(
            Solution::challenge_of_the_keeper(vec_str![".#..", "..##", ".#S.", ".#.T"]),
            -1
        );
        assert_eq!(
            Solution::challenge_of_the_keeper(vec_str!["S###.", "..###", "#..##", "##..#", "###.T"]),
            5
        );
    }
}
