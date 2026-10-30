// https://leetcode.com/problems/two-letter-card-game/
// 3664. Two Letter Words Game
pub struct Solution;
impl Solution {
    pub fn score(cards: Vec<String>, x: char) -> i32 {
        let x = x as u8;
        let mut cntl = [0; 26];
        let mut cntr = [0; 26];
        let mut xx = 0;
        let mut xl = 0;
        let mut ml = 0;
        let mut xr = 0;
        let mut mr = 0;
        for c in cards {
            let c = c.as_bytes();
            if c[0] == x && c[1] == x {
                xx += 1;
            } else if c[0] == x {
                xl += 1;
                cntl[(c[1] - b'a') as usize] += 1;
                ml = ml.max(cntl[(c[1] - b'a') as usize]);
            } else if c[1] == x {
                xr += 1;
                cntr[(c[0] - b'a') as usize] += 1;
                mr = mr.max(cntr[(c[0] - b'a') as usize]);
            }
        }
        let mut ans = 0;
        if ml > xl - ml {
            ans += xl - ml;
            xl = ml * 2 - xl;
        } else {
            ans += xl / 2;
            xl = xl % 2;
        }
        if mr > xr - mr {
            ans += xr - mr;
            xr = mr * 2 - xr;
        } else {
            ans += xr / 2;
            xr = xr % 2;
        }
        let xx_use = (xl + xr).min(xx);
        xx -= xx_use;
        ans = (ans * 2).min(ans + xx / 2);
        ans + xx_use
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn two_letter_words_game() {
        assert_eq!(Solution::score(vec_str!["aa", "ab", "ba", "ac"], 'a'), 2);
        assert_eq!(Solution::score(vec_str!["aa", "ab", "ba"], 'a'), 1);
        assert_eq!(Solution::score(vec_str!["aa", "ab", "ba", "ac"], 'b'), 0);
    }
}
