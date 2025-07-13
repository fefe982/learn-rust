// https://leetcode.com/problems/tiling-a-rectangle-with-the-fewest-squares/
// 1240. Tiling a Rectangle with the Fewest Squares
pub struct Solution;
impl Solution {
    fn check(rec: &Vec<Vec<bool>>, x: usize, y: usize, l: usize) -> bool {
        for i in 0..l {
            for j in 0..l {
                if rec[x + i][y + j] {
                    return false;
                }
            }
        }
        return true;
    }
    fn fill(rec: &mut Vec<Vec<bool>>, x: usize, y: usize, l: usize, val: bool) {
        for i in 0..l {
            for j in 0..l {
                rec[x + i][y + j] = val;
            }
        }
    }
    fn next(rec: &Vec<Vec<bool>>, mut x: usize, mut y: usize) -> (usize, usize) {
        loop {
            y += 1;
            if y >= rec[x].len() {
                x += 1;
                y = 0;
            }
            if x >= rec.len() {
                break;
            }
            if !rec[x][y] {
                break;
            }
        }
        (x, y)
    }
    fn tile(
        rec: &mut Vec<Vec<bool>>,
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        cnt: i32,
        mut min: i32,
    ) -> i32 {
        if cnt == min {
            return cnt;
        }
        let ll = std::cmp::min(n - x, m - y);
        for l in (1..=ll).rev() {
            if !Self::check(rec, x, y, l) {
                continue;
            }
            Self::fill(rec, x, y, l, true);
            let (nx, ny) = Self::next(rec, x, y);
            let ncnt = if nx < rec.len() {
                Self::tile(rec, n, m, nx, ny, cnt + 1, min)
            } else {
                cnt + 1
            };
            if ncnt < min {
                min = ncnt;
            }
            Self::fill(rec, x, y, l, false);
        }
        min
    }
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        Self::tile(&mut vec![vec![false; m]; n], n, m, 0, 0, 0, i32::MAX)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tiling_rectangle() {
        assert_eq!(Solution::tiling_rectangle(2, 3), 3);
        assert_eq!(Solution::tiling_rectangle(5, 8), 5);
        assert_eq!(Solution::tiling_rectangle(11, 13), 6);
    }
}
