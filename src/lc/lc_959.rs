// https://leetcode.com/problems/regions-cut-by-slashes/
// 959. Regions Cut By Slashes
pub struct Solution;
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let r = grid.len();
        let c = grid[0].len();
        let mut uf = (0..r * c * 4).collect::<Vec<_>>();
        let parent = |uf: &mut Vec<usize>, i: usize| {
            let mut j = i;
            while uf[j] != j {
                j = uf[j];
            }
            uf[i] = j;
            j
        };
        let union = |uf: &mut Vec<usize>, i: usize, j: usize| {
            let pi = parent(uf, i);
            let pj = parent(uf, j);
            if pi != pj {
                uf[pi] = pj;
            }
        };
        for (i, row) in grid.iter().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                let base = (i * c + j) * 4;
                if ch == '/' || ch == ' ' {
                    union(&mut uf, base + 0, base + 1);
                    union(&mut uf, base + 2, base + 3);
                }
                if ch == '\\' || ch == ' ' {
                    union(&mut uf, base + 0, base + 2);
                    union(&mut uf, base + 1, base + 3);
                }
                if j + 1 < c {
                    union(&mut uf, base + 2, base + 5);
                }
                if i + 1 < r {
                    union(&mut uf, base + 3, base + c * 4);
                }
            }
        }
        let mut cnt = 0;
        for i in 0..r * c * 4 {
            if uf[i] == i {
                cnt += 1;
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn regions_by_slashes() {
        assert_eq!(Solution::regions_by_slashes(vec_str![" /", "/ "]), 2);
        assert_eq!(Solution::regions_by_slashes(vec_str![" /", "  "]), 1);
        assert_eq!(Solution::regions_by_slashes(vec_str!["/\\", "\\/"]), 5);
    }
}
