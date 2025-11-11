// https://leetcode.cn/problems/pond-sizes-lcci/
// 面试题 16.19. 水域大小
pub struct Solution;
impl Solution {
    fn next(x: usize, dx: i32, lx: usize) -> Option<usize> {
        if (dx < 0 && x > 0) || (dx > 0 && x + 1 < lx) || dx == 0 {
            Some((x as i32 + dx) as usize)
        } else {
            None
        }
    }
    pub fn pond_sizes(mut land: Vec<Vec<i32>>) -> Vec<i32> {
        let lx = land.len();
        let ly = land[0].len();
        let mut res = vec![];
        for ix in 0..lx {
            for iy in 0..ly {
                if land[ix][iy] != 0 {
                    continue;
                }
                let mut q = vec![(ix, iy)];
                let mut sz = 0;
                while let Some((x, y)) = q.pop() {
                    if land[x][y] != 0 {
                        continue;
                    }
                    sz += 1;
                    land[x][y] = -1;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            if let (Some(nx), Some(ny)) =
                                (Self::next(x, dx, lx), Self::next(y, dy, ly))
                            {
                                if land[nx][ny] == 0 {
                                    q.push((nx, ny));
                                }
                            }
                        }
                    }
                }
                res.push(sz);
            }
        }
        res.sort_unstable();
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn pond_sizes() {
        assert_eq!(
            Solution::pond_sizes(vec_vec![
                [0, 2, 1, 0],
                [0, 1, 0, 1],
                [1, 1, 0, 1],
                [0, 1, 0, 1]
            ]),
            vec![1, 2, 4]
        );
    }
}
