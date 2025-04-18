// https://leetcode.com/problems/ones-and-zeroes/
// 474. Ones and Zeroes
pub struct Solution;
impl Solution {
    fn find(strs: &Vec<(usize, usize)>, i: usize, o: usize, z: usize, cache: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if i >= strs.len() || (o == 0 && z == 0) {
            return 0;
        }
        if cache[o][z][i] != -1 {
            return cache[o][z][i];
        }
        let mut l = 0;
        if o >= strs[i].0 && z >= strs[i].1 {
            l = Self::find(strs, i + 1, o - strs[i].0, z - strs[i].1, cache) + 1;
        }
        l = l.max(Self::find(strs, i + 1, o, z, cache));
        cache[o][z][i] = l;
        l
    }
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let strs = strs
            .into_iter()
            .map(|x| {
                x.chars().fold(
                    (0usize, 0usize),
                    |(o, z), c| if c == '1' { (o + 1, z) } else { (o, z + 1) },
                )
            })
            .collect::<Vec<(usize, usize)>>();
        let m = m as usize;
        let n = n as usize;
        Self::find(&strs, 0, n, m, &mut vec![vec![vec![-1; strs.len()]; m + 1]; n + 1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_max_form() {
        assert_eq!(Solution::find_max_form(vec_str!["10", "0", "01", "1"], 3, 27), 4);
        assert_eq!(
            Solution::find_max_form(vec_str!["10", "0001", "111001", "1", "0"], 5, 3),
            4
        );
        assert_eq!(Solution::find_max_form(vec_str!["10", "0", "1"], 1, 1), 2);
    }
}
