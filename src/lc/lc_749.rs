// https://leetcode.com/problems/contain-virus/
// 749. Contain Virus
pub struct Solution;
impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        let mut is_infected = is_infected;
        let m = is_infected.len();
        let n = is_infected[0].len();
        let mut all_walls = 0;
        loop {
            let mut visited = std::collections::HashSet::new();
            let mut regions = vec![];
            for i in 0..m {
                for j in 0..n {
                    if is_infected[i][j] == 1 {
                        if visited.contains(&(i, j)) {
                            continue;
                        }
                        let mut q = vec![(i, j)];
                        let mut wall = 0;
                        let mut infect = std::collections::HashSet::new();
                        while let Some((ci, cj)) = q.pop() {
                            if visited.contains(&(ci, cj)) {
                                continue;
                            }
                            visited.insert((ci, cj));
                            for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                                let ni = (ci as i32 + di) as usize;
                                let nj = (cj as i32 + dj) as usize;
                                if ni >= m || nj >= n {
                                    continue;
                                }
                                if is_infected[ni][nj] == 1 {
                                    q.push((ni, nj));
                                } else if is_infected[ni][nj] == 0 {
                                    infect.insert((ni, nj));
                                    wall += 1;
                                }
                            }
                        }
                        regions.push((infect, wall, i, j));
                    }
                }
            }
            if regions.is_empty() {
                break;
            }
            regions.sort_by_key(|(s, _, _, _)| s.len());
            if let Some((infect, wall, i, j)) = regions.pop() {
                if infect.is_empty() {
                    break;
                }
                all_walls += wall;
                let mut q = vec![(i, j)];
                while let Some((ci, cj)) = q.pop() {
                    is_infected[ci][cj] = 2;
                    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let ni = (ci as i32 + di) as usize;
                        let nj = (cj as i32 + dj) as usize;
                        if ni >= m || nj >= n {
                            continue;
                        }
                        if is_infected[ni][nj] == 1 {
                            q.push((ni, nj));
                        }
                    }
                }
            }
            for (infect, _, _, _) in regions {
                for (i, j) in infect {
                    is_infected[i][j] = 1;
                }
            }
        }
        all_walls
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_contain_virus() {
        assert_eq!(
            Solution::contain_virus(vec_vec![
                [0, 1, 0, 0, 0, 0, 0, 1],
                [0, 1, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 0]
            ]),
            10
        );
        assert_eq!(Solution::contain_virus(vec_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]]), 4);
        assert_eq!(
            Solution::contain_virus(vec_vec![
                [1, 1, 1, 0, 0, 0, 0, 0, 0],
                [1, 0, 1, 0, 1, 1, 1, 1, 1],
                [1, 1, 1, 0, 0, 0, 0, 0, 0]
            ]),
            13
        );
    }
}
