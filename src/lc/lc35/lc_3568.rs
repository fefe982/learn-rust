// https://leetcode.com/problems/minimum-moves-to-clean-the-classroom/
// 3568. Minimum Moves to Clean the Classroom
pub struct Solution;
impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        let classroom = classroom
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut id = vec![vec![0; classroom[0].len()]; classroom.len()];
        let mut full_mask = 0;
        let mut sx = 0;
        let mut sy = 0;
        let mut idl = 0;
        for (i, row) in classroom.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 'S' {
                    sx = i;
                    sy = j;
                }
                if c == 'L' {
                    id[i][j] = idl;
                    idl += 1;
                    full_mask |= 1 << id[i][j];
                }
            }
        }
        let mut eng = vec![vec![vec![-1; full_mask as usize + 1]; classroom[0].len()]; classroom.len()];
        eng[sx][sy][0] = energy;
        let mut q = std::collections::VecDeque::new();
        q.push_back((sx, sy, 0, 0, energy));
        while let Some((x, y, mask, step, e)) = q.pop_front() {
            if mask == full_mask {
                return step;
            }
            if e == 0 {
                continue;
            }
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = ((x as i32 + dir.0) as usize, (y as i32 + dir.1) as usize);
                if nx >= classroom.len() || ny >= classroom[0].len() || classroom[nx][ny] == 'X' {
                    continue;
                }
                let nstep = step + 1;
                let mut nmask = mask;
                if classroom[nx][ny] == 'L' {
                    nmask |= 1 << id[nx][ny];
                }
                let mut ne = e - 1;
                if classroom[nx][ny] == 'R' {
                    ne = energy;
                }
                if ne > eng[nx][ny][nmask] {
                    eng[nx][ny][nmask] = ne;
                    q.push_back((nx, ny, nmask, nstep, ne));
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_moves() {
        assert_eq!(Solution::min_moves(vec_str!["S.", "XL"], 2), 2);
        assert_eq!(Solution::min_moves(vec_str!["LS", "RL"], 4), 3);
        assert_eq!(Solution::min_moves(vec_str!["L.S", "RXL"], 3), -1);
    }
}
