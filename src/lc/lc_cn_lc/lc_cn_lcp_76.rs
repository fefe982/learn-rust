// https://leetcode.cn/problems/1ybDKD/
// LCP 76. 魔法棋盘
pub struct Solution;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    EMPTY,
    R,
    B,
    RR,
    RB,
    BR,
    BB,
    ILL,
}
impl State {
    fn from(c: usize) -> Self {
        match c {
            0 => State::EMPTY,
            1 => State::R,
            2 => State::B,
            3 => State::RR,
            4 => State::RB,
            5 => State::BR,
            6 => State::BB,
            _ => State::ILL,
        }
    }
}
impl Solution {
    fn next(s: State, c: char) -> State {
        match (s, c) {
            (State::EMPTY, 'R') => State::R,
            (State::EMPTY, 'B') => State::B,
            (State::R, 'R') => State::RR,
            (State::R, 'B') => State::RB,
            (State::B, 'R') => State::BR,
            (State::B, 'B') => State::BB,
            (State::RR, 'R') => State::RR,
            (State::RR, 'B') => State::ILL,
            (State::RB, 'R') => State::BR,
            (State::RB, 'B') => State::ILL,
            (State::BR, 'R') => State::ILL,
            (State::BR, 'B') => State::RB,
            (State::BB, 'R') => State::ILL,
            (State::BB, 'B') => State::BB,
            (_, '.') => s,
            _ => State::ILL,
        }
    }
    fn search(
        x: usize,
        y: usize,
        sr: State,
        sc: usize,
        board: &Vec<Vec<char>>,
        cache: &mut Vec<Vec<Vec<Vec<i64>>>>,
    ) -> i64 {
        if x == board.len() {
            return 1;
        }
        if cache[x][y][sr as usize][sc] != -1 {
            return cache[x][y][sr as usize][sc];
        }
        let mut ny = y + 1;
        let mut nx = x;
        if ny == board[0].len() {
            ny = 0;
            nx += 1;
        }
        let mut r = 0;
        let mut base = 1;
        for _ in 0..y {
            base *= 7;
        }
        for nc in ['.', 'R', 'B'] {
            if nc == board[x][y] || board[x][y] == '?' {
                let mut nsr = Self::next(sr, nc);
                let csc = State::from(sc / base % 7);
                let nsc = Self::next(csc, nc);
                if nsr != State::ILL && nsc != State::ILL {
                    if ny == 0 {
                        nsr = State::EMPTY;
                    }
                    let nnsc = sc - csc as usize * base + nsc as usize * base;
                    r += Self::search(nx, ny, nsr, nnsc, board, cache);
                }
            }
        }
        cache[x][y][sr as usize][sc] = r;
        r
    }
    pub fn get_scheme_count(n: i32, m: i32, chessboard: Vec<String>) -> i64 {
        let mut n = n as usize;
        let mut m = m as usize;
        let mut board;
        if n <= m {
            board = vec![vec![]; m];
            for x in chessboard.into_iter() {
                for (j, c) in x.chars().enumerate() {
                    board[j].push(c);
                }
            }
        } else {
            (m, n) = (n, m);
            board = chessboard
                .into_iter()
                .map(|x| x.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
        }
        let mut n7 = 1usize;
        for _ in 0..n {
            n7 *= 7;
        }
        let mut cache = vec![vec![vec![vec![-1; n7]; 7]; n]; m];
        Self::search(0, 0, State::EMPTY, 0, &board, &mut cache)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn get_scheme_count() {
        assert_eq!(Solution::get_scheme_count(2, 3, vec_str!["?B?", ".?R"]), 21);
        assert_eq!(Solution::get_scheme_count(3, 3, vec_str!["..R", "..B", "?R?"]), 5);
        assert_eq!(Solution::get_scheme_count(3, 3, vec_str!["?R?", "B?B", "?R?"]), 105);
    }
}
