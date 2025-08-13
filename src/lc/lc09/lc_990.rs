// https://leetcode.com/problems/satisfiability-of-equality-equations/
// 990. Satisfiability of Equality Equations
pub struct Solution;
impl Solution {
    fn find(uf: &mut Vec<usize>, x: usize) -> usize {
        if uf[x] != x {
            uf[x] = Self::find(uf, uf[x]);
        }
        uf[x]
    }
    fn union(uf: &mut Vec<usize>, x: usize, y: usize) {
        let ix = Self::find(uf, x);
        let iy = Self::find(uf, y);
        if ix != iy {
            uf[ix] = iy;
        }
    }
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut uf = (0..26).collect::<Vec<_>>();
        for eq in equations.iter() {
            if eq.chars().nth(1) == Some('=') {
                let (x, y) = (
                    eq.chars().nth(0).unwrap() as usize - 'a' as usize,
                    eq.chars().nth(3).unwrap() as usize - 'a' as usize,
                );
                Self::union(&mut uf, x, y);
            }
        }
        for eq in equations {
            if eq.chars().nth(1) == Some('!') {
                let (x, y) = (
                    eq.chars().nth(0).unwrap() as usize - 'a' as usize,
                    eq.chars().nth(3).unwrap() as usize - 'a' as usize,
                );
                if Self::find(&mut uf, x) == Self::find(&mut uf, y) {
                    return false;
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn equations_possible() {
        assert_eq!(Solution::equations_possible(vec_str!["a==b", "b!=a"]), false);
        assert_eq!(Solution::equations_possible(vec_str!["b==a", "a==b"]), true);
    }
}
