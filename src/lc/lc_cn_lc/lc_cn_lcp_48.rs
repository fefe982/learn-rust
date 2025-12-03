// https://leetcode.cn/problems/fsa7oZ/
// LCP 48. 无限棋局
pub struct Solution;
impl Solution {
    pub fn gobang(pieces: Vec<Vec<i32>>) -> String {
        let mut board = std::collections::HashMap::<(i32, i32), i32>::with_capacity(pieces.len() + 1);
        let mut pieces = pieces;
        for p in &mut pieces {
            p[2] += 1;
            board.insert((p[0], p[1]), p[2]);
        }
        let dir = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
        let check_win = |board: &std::collections::HashMap<(i32, i32), i32>, x: i32, y: i32, color: i32| {
            for i in 0..4 {
                let mut cnt = 1;
                for j in 0..2 {
                    let (dx, dy) = dir[j * 4 + i];
                    let (mut nx, mut ny) = (x + dx, y + dy);
                    while board.get(&(nx, ny)).is_some_and(|&c| c == color) {
                        cnt += 1;
                        nx += dx;
                        ny += dy;
                    }
                }
                if cnt >= 5 {
                    return true;
                }
            }
            false
        };
        for p in &pieces {
            if p[2] == 2 {
                continue;
            }
            let (x, y) = (p[0], p[1]);
            for (dx, dy) in dir {
                let (nx, ny) = (x + dx, y + dy);
                if board.get(&(nx, ny)).is_none() && check_win(&board, nx, ny, 1) {
                    return "Black".to_string();
                }
            }
        }
        let mut whites = std::collections::HashSet::<(i32, i32)>::new();
        for p in &pieces {
            if p[2] == 1 {
                continue;
            }
            let (x, y) = (p[0], p[1]);
            for (dx, dy) in dir {
                let (nx, ny) = (x + dx, y + dy);
                if board.get(&(nx, ny)).is_none() && check_win(&board, nx, ny, 2) {
                    whites.insert((nx, ny));
                    if whites.len() > 1 {
                        return "White".to_string();
                    }
                }
            }
        }
        if whites.len() == 1 {
            let p = whites.into_iter().next().unwrap();
            board.insert(p, 1);
            let mut blacks = std::collections::HashSet::<(i32, i32)>::new();
            for (&(x, y), &c) in &board {
                if c == 2 {
                    continue;
                }
                for (dx, dy) in dir {
                    let (nx, ny) = (x + dx, y + dy);
                    if board.get(&(nx, ny)).is_none() && check_win(&board, nx, ny, 1) {
                        blacks.insert((nx, ny));
                        if blacks.len() > 1 {
                            return "Black".to_string();
                        }
                    }
                }
            }
            return "None".to_string();
        }
        let check_win_b = |board: &std::collections::HashMap<(i32, i32), i32>, sx: i32, sy: i32| {
            let mut blacks = std::collections::HashSet::<(i32, i32)>::new();
            for i in 0..8 {
                let (dx, dy) = dir[i];
                let mut nx = sx;
                let mut ny = sy;
                for _ in 0..5 {
                    nx += dx;
                    ny += dy;
                    if board.contains_key(&(nx, ny)) {
                        continue;
                    }
                    let mut cnt = 1;
                    for j in 0..2 {
                        let (dx, dy) = dir[i ^ (j * 4)];
                        let mut cx = nx + dx;
                        let mut cy = ny + dy;
                        while board.get(&(cx, cy)).is_some_and(|c| *c == 1) {
                            cnt += 1;
                            cx += dx;
                            cy += dy;
                        }
                    }
                    if cnt >= 5 {
                        blacks.insert((nx, ny));
                        if blacks.len() > 1 {
                            return true;
                        }
                    }
                }
            }
            false
        };
        let mut v = std::collections::HashSet::<(i32, i32)>::new();
        for p in &pieces {
            if p[2] == 2 {
                continue;
            }
            let (x, y) = (p[0], p[1]);
            for dx in -2..=2 {
                for dy in -2..=2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let (nx, ny) = (x + dx, y + dy);
                    if v.contains(&(nx, ny)) || board.contains_key(&(nx, ny)) {
                        continue;
                    }
                    board.insert((nx, ny), 1);
                    v.insert((nx, ny));
                    if check_win_b(&board, nx, ny) {
                        return "Black".to_string();
                    }
                    board.remove(&(nx, ny));
                }
            }
        }
        "None".to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn gobang() {
        assert_eq!(
            Solution::gobang(vec_vec![
                [1, 9, 1],
                [3, 2, 1],
                [6, 9, 0],
                [1, 5, 0],
                [4, 7, 1],
                [5, 7, 1],
                [7, 5, 0],
                [6, 4, 1],
                [2, 7, 1],
                [0, 3, 0],
                [3, 4, 0],
                [8, 4, 0],
                [0, 6, 0],
                [8, 5, 1],
                [3, 3, 1],
                [2, 8, 1],
                [4, 4, 0],
                [5, 9, 0],
                [0, 1, 1],
                [5, 1, 1],
                [6, 3, 0],
                [9, 4, 1],
                [2, 6, 0],
                [0, 8, 1],
                [7, 3, 0],
                [9, 5, 0],
                [9, 2, 0],
                [8, 3, 0],
                [9, 0, 0],
                [8, 9, 0],
                [4, 3, 1],
                [2, 3, 1],
                [7, 4, 1],
                [2, 1, 0],
                [6, 5, 1],
                [8, 8, 0],
                [6, 1, 1],
                [6, 6, 1],
                [1, 3, 1],
                [4, 6, 1],
                [7, 1, 1],
                [5, 5, 0],
                [2, 5, 0]
            ]),
            "None"
        );
        assert_eq!(
            Solution::gobang(vec_vec![
                [0, 0, 0],
                [0, 6, 0],
                [1, 1, 0],
                [1, 5, 0],
                [3, 0, 1],
                [3, 1, 1],
                [3, 2, 1],
                [3, 4, 1],
                [4, 2, 0],
                [4, 4, 0],
                [3, -1, 1]
            ]),
            "White"
        );
        assert_eq!(Solution::gobang(vec_vec![[0, 0, 1], [1, 1, 1], [2, 2, 0]]), "None");
        assert_eq!(
            Solution::gobang(vec_vec![
                [1, 2, 1],
                [1, 4, 1],
                [1, 5, 1],
                [2, 1, 0],
                [2, 3, 0],
                [2, 4, 0],
                [3, 2, 1],
                [3, 4, 0],
                [4, 2, 1],
                [5, 2, 1]
            ]),
            "Black"
        );
    }
}
