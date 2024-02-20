// https://leetcode.com/problems/verbal-arithmetic-puzzle/
// 1307. Verbal Arithmetic Puzzle
pub struct Solution;
impl Solution {
    fn solve(
        words: &Vec<Vec<usize>>,
        result: &Vec<usize>,
        digit: usize,
        sum: usize,
        i2c: &mut Vec<usize>,
        c2i: &mut Vec<usize>,
    ) -> bool {
        if digit >= result.len() {
            return sum == 0;
        }
        let mut lc = vec![];
        let mut li = vec![];
        for i in 0..words.len() {
            if digit < words[i].len() && c2i[words[i][digit]] == usize::MAX {
                lc.push(words[i][digit]);
                c2i[words[i][digit]] = 0;
            }
        }
        'l: loop {
            if !li.is_empty() {
                'next: while !li.is_empty() {
                    let j = li.len() - 1;
                    let c = lc[j];
                    let i = li.pop().unwrap();
                    i2c[i] = usize::MAX;
                    c2i[c] = usize::MAX;
                    for ii in i + 1..10 {
                        if i2c[ii] == usize::MAX {
                            i2c[ii] = c;
                            c2i[c] = ii;
                            li.push(ii);
                            break 'next;
                        }
                    }
                }
                if li.len() == 0 {
                    return false;
                }
            }
            if li.len() < lc.len() {
                for i in 0..10 {
                    if i2c[i] == usize::MAX {
                        li.push(i);
                        i2c[i] = lc[li.len() - 1];
                        c2i[lc[li.len() - 1]] = i;
                        if li.len() == lc.len() {
                            break;
                        }
                    }
                }
            }
            let mut s = sum;
            for iw in 0..words.len() {
                if digit < words[iw].len() {
                    if digit != 0 && digit == words[iw].len() - 1 && c2i[words[iw][digit]] == 0 {
                        if lc.len() == 0 {
                            return false;
                        } else {
                            continue 'l;
                        }
                    }
                    s += c2i[words[iw][digit]];
                }
            }
            let rc = result[digit];
            if c2i[rc] == usize::MAX {
                if i2c[s % 10] == usize::MAX {
                    if digit == 0 || digit != result.len() - 1 || s % 10 != 0 {
                        c2i[rc] = s % 10;
                        i2c[s % 10] = rc;
                        if Self::solve(words, result, digit + 1, s / 10, i2c, c2i) {
                            return true;
                        }
                        c2i[rc] = usize::MAX;
                        i2c[s % 10] = usize::MAX;
                    }
                }
            } else if s % 10 == c2i[rc] && (digit == 0 || digit != result.len() - 1 || s % 10 != 0) {
                if Self::solve(words, result, digit + 1, s / 10, i2c, c2i) {
                    return true;
                }
            }
            if lc.len() == 0 {
                return false;
            }
        }
    }
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let words = words
            .into_iter()
            .map(|x| x.bytes().map(|x| (x - b'A') as usize).rev().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let result = result.bytes().map(|x| (x - b'A') as usize).rev().collect::<Vec<_>>();
        if words.iter().map(|x| x.len()).max().unwrap() > result.len() {
            return false;
        }
        Self::solve(
            &words,
            &result,
            0,
            0,
            &mut vec![usize::MAX; 10],
            &mut vec![usize::MAX; 26],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_solvable() {
        assert_eq!(Solution::is_solvable(vec_str!["WE", "ARE"], "IT".to_string()), false);
        assert_eq!(Solution::is_solvable(vec_str!["A", "B"], "A".to_string()), true);
        assert_eq!(
            Solution::is_solvable(vec_str!["SEND", "MORE"], "MONEY".to_string()),
            true
        );
        assert_eq!(
            Solution::is_solvable(vec_str!["SIX", "SEVEN", "SEVEN"], "TWENTY".to_string()),
            true
        );
        assert_eq!(
            Solution::is_solvable(vec_str!["LEET", "CODE"], "POINT".to_string()),
            false
        );
    }
}
