// https://leetcode.com/problems/pyramid-transition-matrix/
// 756. Pyramid Transition Matrix
pub struct Solution;
impl Solution {
    fn dfs(l: &mut Vec<Vec<usize>>, a: &[[i32; 26]; 26], i: usize, j: usize) -> bool {
        let mut m = a[l[i - 1][j]][l[i - 1][j + 1]];
        if i == l.len() - 1 {
            return m != 0;
        }
        while m != 0 {
            let c = m.trailing_zeros() as usize;
            m -= 1 << c;
            l[i][j] = c;
            let mut ni = i;
            let mut nj = j;
            if j > 0 {
                ni += 1;
                nj -= 1;
            } else {
                ni = 1;
                nj = i;
            }
            if Self::dfs(l, a, ni, nj) {
                return true;
            }
        }
        false
    }
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let l = bottom.chars().map(|c| (c as u8 - b'A') as usize).collect::<Vec<_>>();
        let mut a = [[0; 26]; 26];
        for s in allowed {
            let c = s.as_bytes();
            a[(c[0] - b'A') as usize][(c[1] - b'A') as usize] |= 1 << (c[2] - b'A') as i32;
        }
        let mut p = vec![vec![0; l.len()]; l.len()];
        p[0] = l;
        Self::dfs(&mut p, &a, 1, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn pyramid_transition() {
        assert_eq!(
            Solution::pyramid_transition("BCD".to_string(), vec_str!["BCC", "CDE", "CEA", "FFF"]),
            true
        );
        assert_eq!(
            Solution::pyramid_transition("AAAA".to_string(), vec_str!["AAB", "AAC", "BCD", "BBE", "DEF"]),
            false
        );
    }
}
