// https://leetcode.com/problems/number-of-enclaves/
// 1020. Number of Enclaves
pub struct Solution;
impl Solution {
    fn check(
        grid: &mut Vec<Vec<i32>>,
        y: i32,
        x: i32,
        closed: &mut bool,
        stack: &mut Vec<(usize, usize)>,
    ) {
        if y < 0 || y as usize >= grid.len() || x < 0 || x as usize >= grid[y as usize].len() {
            *closed = false;
            return;
        }
        let x = x as usize;
        let y = y as usize;
        if grid[y][x] == 1 {
            stack.push((y, x));
        }
    }
    pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 1 {
                    let mut cells = 0;
                    let mut stack = vec![(y, x)];
                    let mut closed = true;
                    while let Some((iy, ix)) = stack.pop() {
                        if grid[iy][ix] == 0 {
                            continue;
                        }
                        grid[iy][ix] = 0;
                        let iy = iy as i32;
                        let ix = ix as i32;
                        Self::check(&mut grid, iy - 1, ix, &mut closed, &mut stack);
                        Self::check(&mut grid, iy + 1, ix, &mut closed, &mut stack);
                        Self::check(&mut grid, iy, ix - 1, &mut closed, &mut stack);
                        Self::check(&mut grid, iy, ix + 1, &mut closed, &mut stack);
                        cells += 1;
                    }
                    if closed {
                        cnt += cells;
                    }
                }
            }
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_enclaves() {
        assert_eq!(
            Solution::num_enclaves(vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            3
        );
        assert_eq!(
            Solution::num_enclaves(vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0]
            ]),
            0
        );
    }
}
