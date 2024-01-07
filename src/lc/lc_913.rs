// https://leetcode.com/problems/cat-and-mouse/
// 913. Cat and Mouse
pub struct Solution;
impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        #[derive(Copy, Clone, Eq, PartialEq)]
        enum State {
            DRAW,
            MOUSE,
            CAT,
            ILL,
        }
        let mut s = vec![vec![vec![State::ILL; n]; n]; 2];
        for i in 1..n {
            s[0][i][0] = State::MOUSE;
            for j in 1..n {
                s[0][i][j] = if i == j { State::CAT } else { State::DRAW };
                s[1][i][j] = if i == j { State::CAT } else { State::DRAW };
            }
        }
        loop {
            let mut change = false;
            for c in 1..n {
                for m in 0..n {
                    for t in 0..2 {
                        if s[t][c][m] != State::DRAW {
                            continue;
                        }
                        let mut r = 0;
                        let n_state = if t == 0 {
                            for &nc in &graph[c] {
                                r |= 1 << (s[1 - t][nc as usize][m] as i32);
                            }
                            if (r & (1 << State::CAT as i32)) != 0 {
                                State::CAT
                            } else if (r & (1 << State::DRAW as i32)) != 0 {
                                State::DRAW
                            } else {
                                State::MOUSE
                            }
                        } else {
                            for &nm in &graph[m] {
                                r |= 1 << (s[1 - t][c][nm as usize] as i32);
                            }
                            if (r & (1 << State::MOUSE as i32)) != 0 {
                                State::MOUSE
                            } else if (r & (1 << State::DRAW as i32)) != 0 {
                                State::DRAW
                            } else {
                                State::CAT
                            }
                        };
                        if n_state != s[t][c][m] {
                            change = true;
                        }
                        s[t][c][m] = n_state;
                    }
                }
            }
            if s[1][2][1] != State::DRAW || change == false {
                return s[1][2][1] as i32;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_cat_mouse_game() {
        assert_eq!(
            Solution::cat_mouse_game(vec_vec![[2, 5], [3], [0, 4, 5], [1, 4, 5], [2, 3], [0, 2, 3]]),
            0
        );
        assert_eq!(Solution::cat_mouse_game(vec_vec![[1, 3], [0], [3], [0, 2]]), 1);
    }
}
