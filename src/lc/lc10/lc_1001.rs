// https://leetcode.com/problems/grid-illumination/
// 1001. Grid Illumination
pub struct Solution;
impl Solution {
    fn remove(map: &mut std::collections::HashMap<i32, std::collections::HashSet<i32>>, key: i32, val: i32) {
        if let Some(set) = map.get_mut(&key) {
            set.remove(&val);
            if set.is_empty() {
                map.remove(&key);
            }
        }
    }
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
        let mut col = row.clone();
        let mut diag1 = row.clone();
        let mut diag2 = row.clone();
        for v in lamps {
            row.entry(v[0]).or_default().insert(v[1]);
            col.entry(v[1]).or_default().insert(v[0]);
            diag1.entry(v[0] + v[1]).or_default().insert(v[0]);
            diag2.entry(v[0] - v[1]).or_default().insert(v[0]);
        }
        let mut ans = vec![];
        for q in queries {
            let x = q[0];
            let y = q[1];
            ans.push(
                if row.contains_key(&x)
                    || col.contains_key(&y)
                    || diag1.contains_key(&(x + y))
                    || diag2.contains_key(&(x - y))
                {
                    1
                } else {
                    0
                },
            );
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= n || ny < 0 || ny >= n {
                        continue;
                    }
                    Self::remove(&mut row, nx, ny);
                    Self::remove(&mut col, ny, nx);
                    Self::remove(&mut diag1, nx + ny, nx);
                    Self::remove(&mut diag2, nx - ny, nx);
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
    fn test_grid_illumination() {
        assert_eq!(
            Solution::grid_illumination(5, vec_vec![[0, 0], [4, 4]], vec_vec![[1, 1], [1, 0]]),
            vec![1, 0]
        );
        assert_eq!(
            Solution::grid_illumination(5, vec_vec![[0, 0], [4, 4]], vec_vec![[1, 1], [1, 1]]),
            vec![1, 1]
        );
        assert_eq!(
            Solution::grid_illumination(5, vec_vec![[0, 0], [0, 4]], vec_vec![[0, 4], [0, 1], [1, 4]]),
            vec![1, 1, 0]
        );
    }
}
