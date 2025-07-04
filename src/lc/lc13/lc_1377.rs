// https://leetcode.com/problems/frog-position-after-t-seconds/
// 1377. Frog Position After T Seconds
pub struct Solution;
impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut edge_map = std::collections::HashMap::<i32, Vec<i32>>::new();
        for edge in edges {
            edge_map.entry(edge[0]).or_default().push(edge[1]);
            edge_map.entry(edge[1]).or_default().push(edge[0]);
        }
        let mut visited = vec![false; n as usize + 1];
        let mut q = vec![(1, 0, 1.0)];
        while let Some((i, step, prop)) = q.pop() {
            visited[i as usize] = true;
            let next: Vec<i32> = if let Some(n_nodes) = edge_map.get(&i) {
                (*n_nodes)
                    .iter()
                    .cloned()
                    .filter(|&x| !visited[x as usize])
                    .collect()
            } else {
                Vec::new()
            };
            let n_next = next.len() as f64;
            if i == target {
                if step == t || (step < t && next.len() == 0) {
                    return prop;
                } else {
                    return 0.0;
                }
            }
            for nnode in next {
                q.push((nnode, step + 1, prop / n_next));
            }
        }
        0.0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn frog_position() {
        assert_eq!(
            Solution::frog_position(1, Vec::<Vec<i32>>::new(), 1, 1),
            1.0
        );
        assert_eq!(
            Solution::frog_position(
                8,
                vec_vec![[2, 1], [3, 2], [4, 1], [5, 1], [6, 4], [7, 1], [8, 7]],
                7,
                7
            ),
            0.0
        );
        assert_eq!(
            Solution::frog_position(
                7,
                vec_vec![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]],
                2,
                4
            ),
            1.0 / 6.0
        );
        assert_eq!(
            Solution::frog_position(
                7,
                vec_vec![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]],
                1,
                7
            ),
            1.0 / 3.0
        );
    }
}
