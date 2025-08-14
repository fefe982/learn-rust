// https://leetcode.com/problems/escape-a-large-maze/
// 1036. Escape a Large Maze
pub struct Solution;
impl Solution {
    fn remap(mut map: std::collections::BTreeMap<i32, i32>) -> std::collections::BTreeMap<i32, i32> {
        let mut last_k = 0;
        let mut last_v = 0;
        for (&k, v) in map.iter_mut() {
            if k == last_k {
                *v = last_v;
            } else if k == last_k + 1 {
                *v = last_v + 1;
            } else {
                *v = last_v + 2;
            }
            last_k = k;
            last_v = *v;
        }
        map
    }
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut map0 = std::collections::BTreeMap::<i32, i32>::new();
        map0.insert(0, 0);
        map0.insert(999999, 0);
        let mut map1 = map0.clone();
        map0.insert(source[0], 0);
        map0.insert(target[0], 0);
        map1.insert(source[1], 0);
        map1.insert(target[1], 0);
        for b in &blocked {
            map0.insert(b[0], 0);
            map1.insert(b[1], 0);
        }
        map0 = Self::remap(map0);
        map1 = Self::remap(map1);
        let blocked = blocked
            .into_iter()
            .map(|v| (*map0.get(&v[0]).unwrap(), *map1.get(&v[1]).unwrap()))
            .collect::<std::collections::HashSet<_>>();
        let source = (*map0.get(&source[0]).unwrap(), *map1.get(&source[1]).unwrap());
        let target = (*map0.get(&target[0]).unwrap(), *map1.get(&target[1]).unwrap());
        let bound = (*map0.get(&999999).unwrap(), *map1.get(&999999).unwrap());
        let mut visited = vec![vec![false; bound.1 as usize + 1]; bound.0 as usize + 1];
        let mut q = vec![source];
        visited[source.0 as usize][source.1 as usize] = true;
        while let Some(n) = q.pop() {
            let (x, y) = (n.0, n.1);
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx > bound.0 || ny < 0 || ny > bound.1 {
                    continue;
                }
                if blocked.contains(&(nx, ny)) {
                    continue;
                }
                if visited[nx as usize][ny as usize] {
                    continue;
                }
                if (nx, ny) == target {
                    return true;
                }
                visited[nx as usize][ny as usize] = true;
                q.push((nx, ny));
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_is_escape_possible() {
        assert_eq!(
            Solution::is_escape_possible(vec_vec![[0, 1], [1, 0]], vec![0, 0], vec![0, 2]),
            false
        );
        assert_eq!(
            Solution::is_escape_possible(vec_vec![], vec![0, 0], vec![999999, 999999]),
            true
        );
    }
}
