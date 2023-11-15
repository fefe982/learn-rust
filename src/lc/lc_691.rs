// https://leetcode.com/problems/stickers-to-spell-word/
// 691. Stickers to Spell Word
pub struct Solution;
impl Solution {
    fn count(
        scnt: &Vec<Vec<i32>>,
        tcnt: Vec<(u8, i32)>,
        dp: &mut std::collections::HashMap<Vec<(u8, i32)>, i32>,
    ) -> i32 {
        if tcnt.is_empty() {
            return 0;
        }
        if let Some(&c) = dp.get(&tcnt) {
            return c;
        }
        let mut mincnt = i32::MAX;
        for cnt in scnt.iter() {
            if cnt[tcnt[0].0 as usize] == 0 {
                continue;
            }
            let mut next = vec![];
            for &(c, tc) in tcnt.iter() {
                let nc = (tc - cnt[c as usize]).max(0);
                if nc > 0 {
                    next.push((c, nc))
                }
            }
            let nc = Self::count(scnt, next, dp);
            if nc != -1 {
                mincnt = mincnt.min(nc + 1)
            }
        }
        if mincnt == i32::MAX {
            -1
        } else {
            mincnt
        }
    }
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut scnt = vec![vec![0; 26]; stickers.len()];
        for (i, s) in stickers.iter().enumerate() {
            for c in s.chars() {
                scnt[i][c as usize - 'a' as usize] += 1;
            }
        }
        let mut dp = std::collections::HashMap::<Vec<(u8, i32)>, i32>::new();
        let mut target = target.as_bytes().into_iter().map(|&c| c - b'a').collect::<Vec<_>>();
        target.sort();
        let mut tcnt = vec![];
        let mut c = 0;
        let mut last = 0;
        for cur in target.into_iter().chain(vec![50].into_iter()) {
            if cur != last {
                if c != 0 {
                    tcnt.push((last, c))
                }
                last = cur;
                c = 1;
            } else {
                c += 1;
            }
        }
        Self::count(&scnt, tcnt, &mut dp)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_min_stickers() {
        assert_eq!(
            Solution::min_stickers(vec_str!["with", "example", "science"], "thehat".to_string()),
            3
        );
        assert_eq!(
            Solution::min_stickers(vec_str!["notice", "possible"], "basicbasic".to_string()),
            -1
        );
    }
}
