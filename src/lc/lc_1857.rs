// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/description/
// 1857. Largest Color Value in a Directed Graph
pub struct Solution;
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let colors = colors.as_bytes();
        let mut graph: Vec<Vec<i32>> = vec![vec![]; colors.len()];
        let mut rank = vec![0; colors.len()];
        for edge in edges {
            rank[edge[1] as usize] += 1;
            graph[edge[0] as usize].push(edge[1]);
        }
        let mut visited = 0;
        let mut node_que = Vec::new();
        let mut color_max_cnt = vec![vec![0; 26]; colors.len()];
        for (idx, &r) in rank.iter().enumerate() {
            if r == 0 {
                node_que.push(idx);
            }
            color_max_cnt[idx][(colors[idx] - b'a') as usize] = 1;
        }
        let mut max_cnt = 1;
        while let Some(idx) = node_que.pop() {
            visited += 1;
            for &dst in &graph[idx] {
                let dst = dst as usize;
                let dst_c = (colors[dst as usize] - b'a') as usize;
                for c in 0..26usize {
                    color_max_cnt[dst][c] = std::cmp::max(
                        color_max_cnt[idx][c] + (if dst_c == c { 1 } else { 0 }),
                        color_max_cnt[dst][c],
                    );
                    max_cnt = std::cmp::max(max_cnt, color_max_cnt[dst][c]);
                }
                rank[dst] -= 1;
                if rank[dst] == 0 {
                    node_que.push(dst);
                }
            }
        }
        if visited < colors.len() {
            -1
        } else {
            max_cnt
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_score_indices() {
        assert_eq!(
            Solution::largest_path_value(
                String::from("abaca"),
                [[0, 1], [0, 2], [2, 3], [3, 4]]
                    .into_iter()
                    .map(|x| x.into_iter().collect())
                    .collect()
            ),
            3
        );
        assert_eq!(
            Solution::largest_path_value(String::from("a"), vec![vec![0, 0]]),
            -1
        )
    }
}
