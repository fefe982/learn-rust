// https://leetcode.cn/problems/fHi6rV/
// LCP 41. 黑白翻转棋
pub struct Solution;
impl Solution {
    fn adv(x: usize, dx: i32, lx: usize) -> Option<usize> {
        if dx < 0 && (-dx) as usize > x {
            return None;
        }
        if dx > 0 && x + dx as usize >= lx {
            return None;
        }
        Some((x as i32 + dx) as usize)
    }
    fn flip_board(mut board: Vec<Vec<u8>>, x: usize, y: usize) -> i32 {
        if board[y][x] != 0 {
            return 0;
        }
        board[y][x] = 1;
        let lx = board[0].len();
        let ly = board.len();
        let mut q = std::collections::VecDeque::<(usize, usize)>::new();
        q.push_back((x, y));
        let mut cnt = 0;
        while let Some((x, y)) = q.pop_front() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    for l in 1.. {
                        if let (Some(nx), Some(ny)) =
                            (Self::adv(x, dx * l, lx), Self::adv(y, dy * l, ly))
                        {
                            if board[ny][nx] == 0 {
                                break;
                            } else if board[ny][nx] == 1 {
                                if l > 1 {
                                    cnt += l - 1;
                                    for ll in 1..l {
                                        if let (Some(nx), Some(ny)) =
                                            (Self::adv(x, dx * ll, lx), Self::adv(y, dy * ll, ly))
                                        {
                                            board[ny][nx] = 1;
                                            q.push_back((nx, ny));
                                        }
                                    }
                                }
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        cnt
    }
    pub fn flip_chess(chessboard: Vec<String>) -> i32 {
        let board = chessboard
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => 0,
                        'X' => 1,
                        'O' => 2,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        let lx = board[0].len();
        let ly = board.len();
        let mut m = 0;
        for iy in 0..ly {
            for ix in 0..lx {
                m = m.max(Self::flip_board(board.clone(), ix, iy));
            }
        }
        m
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn flip_chess() {
        assert_eq!(
            Solution::flip_chess(vec_str!["....X.", "....X.", "XOOO..", "......", "......"]),
            3
        );
        assert_eq!(Solution::flip_chess(vec_str![".X.", ".O.", "XO."]), 2);
        assert_eq!(
            Solution::flip_chess(vec_str![
                ".......", ".......", ".......", "X......", ".O.....", "..O....", "....OOX"
            ]),
            4
        );
    }
}
