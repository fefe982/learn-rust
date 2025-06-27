// https://leetcode.com/problems/shortest-path-to-get-all-keys/
// 864. Shortest Path to Get All Keys
pub struct Solution;
impl Solution {
    fn walk(
        grid: &Vec<Vec<u8>>,
        sx: usize,
        sy: usize,
        step: i32,
        key_set: usize,
        key_map: &std::collections::HashMap<u8, i32>,
        nsteps: &mut Vec<Vec<Vec<i32>>>,
        qkey: &mut std::collections::VecDeque<(usize, i32, usize, usize)>,
    ) {
        let csteps = &mut nsteps[key_set];
        if csteps[sy][sx] < step {
            return;
        }
        csteps[sy][sx] = step;
        let mut q = std::collections::VecDeque::new();
        q.push_back((sx, sy));
        let dir = [[0, 1], [1, 0], [-1, 0], [0, -1]];
        let ly = grid.len();
        let lx = grid[0].len();
        while let Some((cx, cy)) = q.pop_front() {
            let cstep = csteps[cy][cx];
            for d in dir {
                if (d[0] < 0 && cx == 0)
                    || (d[1] < 0 && cy == 0)
                    || (d[0] > 0 && cx + 1 >= lx)
                    || (d[1] > 0 && cy + 1 >= ly)
                {
                    continue;
                }
                let nx = (cx as i32 + d[0]) as usize;
                let ny = (cy as i32 + d[1]) as usize;
                match grid[ny][nx] {
                    b'@' | b'.' => {
                        if csteps[ny][nx] > cstep + 1 {
                            csteps[ny][nx] = cstep + 1;
                            q.push_back((nx, ny));
                        }
                    }
                    b'a'..=b'z' => {
                        let nkey = key_map.get(&(grid[ny][nx] - b'a')).unwrap();
                        if key_set & (1 << *nkey) != 0 {
                            if csteps[ny][nx] > cstep + 1 {
                                csteps[ny][nx] = cstep + 1;
                                q.push_back((nx, ny));
                            }
                        } else {
                            qkey.push_back((key_set | (1 << *nkey), cstep + 1, nx, ny));
                        }
                    }
                    b'A'..=b'Z' => {
                        let nkey = key_map.get(&(grid[ny][nx] - b'A')).unwrap();
                        if key_set & (1 << *nkey) != 0 {
                            if csteps[ny][nx] > cstep + 1 {
                                csteps[ny][nx] = cstep + 1;
                                q.push_back((nx, ny));
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<u8>> = grid
            .into_iter()
            .map(|x| x.as_bytes().into_iter().cloned().collect())
            .collect();
        let mut key_map = std::collections::HashMap::new();
        let ly = grid.len();
        let lx = grid[0].len();
        let mut sx = 0;
        let mut sy = 0;
        let mut nkey = 0;
        for x in 0..lx {
            for y in 0..ly {
                match grid[y][x] {
                    b'@' => {
                        sx = x;
                        sy = y;
                    }
                    b'a'..=b'z' => {
                        key_map.insert(grid[y][x] - b'a', nkey);
                        nkey += 1;
                    }
                    _ => (),
                }
            }
        }
        let mut nsteps = vec![vec![vec![i32::MAX; lx]; ly]; 1 << nkey];
        let mut qkey = std::collections::VecDeque::<(usize, i32, usize, usize)>::new();
        qkey.push_back((0, 0, sx, sy));
        let mut min_step = i32::MAX;
        while let Some((key_set, step, cx, cy)) = qkey.pop_front() {
            if key_set == (1 << nkey) - 1 {
                min_step = min_step.min(step);
                continue;
            }
            Self::walk(
                &grid,
                cx,
                cy,
                step,
                key_set,
                &key_map,
                &mut nsteps,
                &mut qkey,
            )
        }
        if min_step == i32::MAX {
            -1
        } else {
            min_step
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn shortest_path_all_keys() {
        assert_eq!(
            Solution::shortest_path_all_keys(vec_str!["@.a..", "###.#", "b.A.B"]),
            8
        );
        assert_eq!(
            Solution::shortest_path_all_keys(vec_str!["@..aA", "..B#.", "....b"]),
            6
        );
        assert_eq!(Solution::shortest_path_all_keys(vec_str!["@Aa"]), -1);
    }
}
