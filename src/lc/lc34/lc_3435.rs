// https://leetcode.com/problems/frequencies-of-shortest-supersequences/
// 3435. Frequencies of Shortest Supersequence
pub struct Solution;
impl Solution {
    fn check(parr: &Vec<i32>, carr: &Vec<i32>, mut dmask: i32) -> bool {
        let mut q = std::collections::VecDeque::new();
        for i in 0..parr.len() {
            if dmask & (1 << i) != 0 {
                continue;
            }
            if parr[i] & !dmask == 0 {
                dmask |= 1 << i;
                q.push_back(i);
            }
        }
        while let Some(i) = q.pop_front() {
            let mut c = carr[i] & !dmask;
            while c > 0 {
                let j = c.trailing_zeros() as usize;
                if (dmask & (1 << j) == 0) && (parr[j] & !dmask == 0) {
                    dmask |= 1 << j;
                    q.push_back(j);
                }
                c &= c - 1;
            }
        }
        dmask == (1 << parr.len()) - 1
    }
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut c2i = vec![26; 26];
        let mut i2c = Vec::with_capacity(26);
        let mut parr = Vec::with_capacity(26);
        let mut carr = Vec::with_capacity(26);
        let mut double = 0;
        for w in words.iter() {
            let wb = w.as_bytes();
            for b in wb.iter() {
                let c = (*b - b'a') as usize;
                if c2i[c] == 26 {
                    c2i[c] = i2c.len();
                    i2c.push(c);
                    parr.push(0);
                    carr.push(0);
                }
            }
            let i0 = c2i[(wb[0] - b'a') as usize];
            let i1 = c2i[(wb[1] - b'a') as usize];
            if i0 != i1 {
                parr[i1] |= 1 << i0;
                carr[i0] |= 1 << i1;
            } else {
                double |= 1 << i0;
            }
        }
        let mut slen = 0;
        let fullmask = (1 << i2c.len()) - 1;
        let smask = fullmask & !double;
        let mut csmask: i32 = smask;
        let mut ans = vec![];
        let get_ans = |csmask: i32| {
            let mut ans = vec![0; 26];
            for i in 0..i2c.len() {
                if csmask & (1 << i) != 0 {
                    ans[i2c[i]] = 1;
                } else {
                    ans[i2c[i]] = 2;
                }
            }
            ans
        };
        loop {
            if csmask.count_ones() >= slen {
                let dmask = fullmask & !csmask;
                if Self::check(&parr, &carr, dmask) {
                    if csmask.count_ones() > slen {
                        slen = csmask.count_ones();
                        ans = vec![get_ans(csmask)];
                    } else {
                        ans.push(get_ans(csmask));
                    }
                }
            }
            if csmask == 0 {
                break;
            }
            csmask = smask & (csmask - 1);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn supersequences() {
        assert_sort_eq!(
            Solution::supersequences(vec_str!["ie", "zi", "ez", "oz", "ce", "eo", "zo"]),
            vec_vec![[0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]]
        );
        assert_sort_eq!(
            Solution::supersequences(vec_str!["ab", "ba"]),
            vec_vec![
                [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ]
        );
        assert_sort_eq!(
            Solution::supersequences(vec_str!["aa", "ac"]),
            vec_vec![[2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
        );
        assert_sort_eq!(
            Solution::supersequences(vec_str!["aa", "bb", "cc"]),
            vec_vec![[2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
        );
    }
}
