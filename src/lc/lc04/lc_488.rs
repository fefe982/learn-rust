// https://leetcode.com/problems/zuma-game/
// 488. Zuma Game
pub struct Solution;
impl Solution {
    fn get_board(board: &Vec<u8>, ins: u8, pos: usize) -> Vec<u8> {
        let mut l = pos;
        let mut r = pos;
        while l > 0 && board[l - 1] == ins {
            l -= 1;
        }
        while r < board.len() && board[r] == ins {
            r += 1;
        }
        if r <= l + 1 {
            let mut b = board.clone();
            b.insert(pos, ins);
            return b;
        }
        while l > 0 && r < board.len() && board[l - 1] == board[r] {
            let mut nl = l;
            let mut nr = r;
            while nl > 0 && board[nl - 1] == board[r] {
                nl -= 1;
            }
            while nr < board.len() && board[nr] == board[r] {
                nr += 1;
            }
            if (l - nl) + (nr - r) >= 3 {
                l = nl;
                r = nr;
            } else {
                break;
            }
        }
        let mut b = vec![];
        b.extend_from_slice(&board[0..l]);
        b.extend_from_slice(&board[r..]);
        return b;
    }
    fn fd_min_step(board: Vec<u8>, hand: &mut Vec<u8>, cache: &mut std::collections::HashMap<Vec<u8>, i32>) -> i32 {
        if board.is_empty() {
            return 0;
        }
        if hand.is_empty() {
            return i32::MAX;
        }
        let mut key = Vec::with_capacity(board.len() + hand.len() + 1);
        key.extend_from_slice(&board[..]);
        key.push(b'.');
        key.extend_from_slice(&hand[..]);
        if let Some(&s) = cache.get(&key) {
            return s;
        }
        let mut s = i32::MAX;
        for ib in 0..=board.len() {
            for ih in 0..hand.len() {
                if ih > 0 && hand[ih] == hand[ih - 1] {
                    continue;
                }
                if ib > 0 && board[ib - 1] == hand[ih] {
                    continue;
                }
                if hand.len() == 1 && (ib == board.len() || board[ib] != hand[0]) {
                    continue;
                }
                let ins = hand.remove(ih);
                let ns = Self::fd_min_step(Self::get_board(&board, ins, ib), hand, cache);
                hand.insert(ih, ins);
                if ns != i32::MAX {
                    s = s.min(ns + 1);
                }
            }
        }
        cache.insert(key, s);
        s
    }
    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut hand = hand.bytes().into_iter().collect::<Vec<u8>>();
        hand.sort_unstable();
        let board = board.bytes().into_iter().collect::<Vec<u8>>();
        let s = Self::fd_min_step(board, &mut hand, &mut std::collections::HashMap::new());
        if s == i32::MAX {
            -1
        } else {
            s
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_min_step() {
        assert_eq!(
            Solution::find_min_step("RRYRRYYRYYRRYYRR".to_owned(), "YYRYY".to_owned()),
            2
        );
        assert_eq!(
            Solution::find_min_step("RRGGBBYYWWRRGGBB".to_owned(), "RGBYW".to_owned()),
            -1
        );
        assert_eq!(
            Solution::find_min_step("BGBBYRYYRBRWYBRR".to_owned(), "YWYRB".to_owned()),
            -1
        );
        assert_eq!(Solution::find_min_step("RRWWRRBBRR".to_owned(), "WB".to_owned()), 2);
        assert_eq!(Solution::find_min_step("WRRBBW".to_owned(), "RB".to_owned()), -1);
        assert_eq!(Solution::find_min_step("WWRRBBWW".to_owned(), "WRBRW".to_owned()), 2);
        assert_eq!(Solution::find_min_step("G".to_owned(), "GGGGGG".to_owned()), 2)
    }
}
