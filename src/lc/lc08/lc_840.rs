// https://leetcode.com/problems/magic-squares-in-grid/
// 840. Magic Squares In Grid
pub struct Solution;
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let nr = grid.len();
        let nc = grid[0].len();
        for ir in 0..nr {
            if ir + 3 > nr {
                break;
            }
            'next: for ic in 0..nc {
                if ic + 3 > nc {
                    break;
                }
                let mut app = [false; 10];
                let mut sumr = [0; 3];
                let mut sumc = [0; 3];
                let mut sumd = [0; 2];
                for dr in 0..3 {
                    for dc in 0..3 {
                        let r = ir + dr;
                        let c = ic + dc;
                        let v = grid[r][c];
                        if v < 1 || v > 9 || app[v as usize] {
                            continue 'next;
                        }
                        app[v as usize] = true;
                        sumr[dr] += v;
                        sumc[dc] += v;
                        if dr == dc {
                            sumd[0] += v;
                        }
                        if dr + dc == 2 {
                            sumd[1] += v;
                        }
                    }
                }
                if sumr == [15, 15, 15] && sumc == [15, 15, 15] && sumd == [15, 15] {
                    ans += 1;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_magic_squares_inside() {
        assert_eq!(
            Solution::num_magic_squares_inside(vec_vec![[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]]),
            1
        );
        assert_eq!(Solution::num_magic_squares_inside(vec_vec![[8]]), 0);
    }
}
