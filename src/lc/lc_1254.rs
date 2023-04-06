// https://leetcode.com/problems/number-of-closed-islands/
// 1254. Number of Closed Islands
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
        if grid[y][x] == 0 {
            stack.push((y, x));
        }
    }
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut cnt = 0;
        let mut idx = 2;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 0 {
                    let mut stack = vec![(y, x)];
                    let mut closed = true;
                    while let Some((iy, ix)) = stack.pop() {
                        grid[iy][ix] = idx;
                        let iy = iy as i32;
                        let ix = ix as i32;
                        Self::check(&mut grid, iy - 1, ix, &mut closed, &mut stack);
                        Self::check(&mut grid, iy + 1, ix, &mut closed, &mut stack);
                        Self::check(&mut grid, iy, ix - 1, &mut closed, &mut stack);
                        Self::check(&mut grid, iy, ix + 1, &mut closed, &mut stack);
                    }
                    idx += 1;
                    if closed {
                        cnt += 1;
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
    fn closed_island() {
        assert_eq!(
            Solution::closed_island(vec![
                vec![1, 1, 1, 1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 1, 0],
                vec![1, 0, 1, 0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::closed_island(vec![
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 1, 1, 1, 0]
            ]),
            1
        );
        assert_eq!(
            Solution::closed_island(vec![
                vec![1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 1, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1]
            ]),
            2
        );
    }
}
