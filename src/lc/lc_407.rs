// https://leetcode.com/problems/trapping-rain-water-ii/
// 407. Trapping Rain Water II
pub struct Solution;
impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let h = height_map.len();
        let w = height_map[0].len();
        let mut fixed = vec![vec![false; w]; h];
        for i in 0..h {
            fixed[i][0] = true;
            fixed[i][w - 1] = true;
        }
        for i in 0..w {
            fixed[0][i] = true;
            fixed[h - 1][i] = true;
        }
        let mut nwater = 0;
        for ih in 1..h - 1 {
            for iw in 1..w - 1 {
                if fixed[ih][iw] {
                    continue;
                }
                let mut visited = vec![vec![false; w]; h];
                let mut q = std::collections::BinaryHeap::new();
                q.push((-height_map[ih][iw], ih, iw));
                visited[ih][iw] = true;
                let mut min_bound = height_map[ih][iw];
                let mut filled = vec![];
                let mut fix = false;
                while let Some((h, ch, cw)) = q.pop() {
                    min_bound = -h;
                    if fixed[ch][cw] {
                        fix = true;
                        break;
                    }
                    filled.push((ch, cw));
                    let mut rise = true;
                    for d in [[0, -1], [-1, 0], [1, 0], [0, 1]] {
                        let nh = (ch as i32 + d[0]) as usize;
                        let nw = (cw as i32 + d[1]) as usize;
                        if !visited[nh][nw] && height_map[nh][nw] < height_map[ch][cw] {
                            rise = false;
                            break;
                        }
                    }
                    if !rise {
                        break;
                    }
                    for d in [[0, -1], [-1, 0], [1, 0], [0, 1]] {
                        let nh = (ch as i32 + d[0]) as usize;
                        let nw = (cw as i32 + d[1]) as usize;
                        if !visited[nh][nw] {
                            visited[nh][nw] = true;
                            q.push((-height_map[nh][nw], nh, nw));
                        }
                    }
                }
                for (ch, cw) in filled {
                    nwater += min_bound - height_map[ch][cw];
                    height_map[ch][cw] = min_bound;
                    if fix {
                        fixed[ch][cw] = true;
                    }
                }
            }
        }
        nwater
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn trap_rain_water() {
        assert_eq!(
            Solution::trap_rain_water(vec_vec![
                [1, 4, 3, 1, 3, 2],
                [3, 2, 1, 3, 2, 4],
                [2, 3, 3, 2, 3, 1]
            ]),
            4
        );
        assert_eq!(
            Solution::trap_rain_water(vec_vec![
                [3, 3, 3, 3, 3],
                [3, 2, 2, 2, 3],
                [3, 2, 1, 2, 3],
                [3, 2, 2, 2, 3],
                [3, 3, 3, 3, 3]
            ]),
            10
        );
    }
}
