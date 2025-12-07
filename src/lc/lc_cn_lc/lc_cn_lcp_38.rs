// https://leetcode.cn/problems/7rLGCR/
// LCP 38. 守卫城堡
pub struct Solution;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
    cap: i32,
    flow: i32,
}
impl Edge {
    fn new(from: usize, to: usize, cap: i32) -> Self {
        Self { from, to, cap, flow: 0 }
    }
}
struct Flow {
    n: usize,
    edges: Vec<Edge>,
    graph: Vec<Vec<usize>>,
}
impl Flow {
    fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec![],
            graph: vec![vec![]; n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, cap: i32) {
        let idx = self.edges.len();
        let rev_idx = self.edges.len() + 1;
        self.graph[from].push(idx);
        self.graph[to].push(rev_idx);
        self.edges.push(Edge::new(from, to, cap));
        self.edges.push(Edge::new(to, from, 0));
    }
    fn max_flow(&mut self, s: usize, t: usize) -> i32 {
        let mut max_flow = 0;
        let mut q = std::collections::VecDeque::with_capacity(self.n);
        let mut iter = vec![0; self.n];
        let mut level = vec![-1; self.n];
        loop {
            q.clear();
            q.push_back((s, 0));
            level.fill(-1);
            level[s] = 0;
            while let Some((n, f)) = q.pop_front() {
                for &e in &self.graph[n] {
                    let edge = &self.edges[e];
                    if edge.cap > edge.flow && level[edge.to] == -1 {
                        level[edge.to] = f + 1;
                        if edge.to == t {
                            break;
                        }
                        q.push_back((edge.to, f + 1));
                    }
                }
            }
            if level[t] == -1 {
                break;
            }
            fn dfs(g: &mut Flow, n: usize, t: usize, level: &mut Vec<i32>, iter: &mut Vec<usize>, flow: i32) -> i32 {
                if n == t {
                    return flow;
                }
                let ln = level[n];
                let mut f = 0;
                for i in iter[n]..g.graph[n].len() {
                    let edge = g.edges[g.graph[n][i]];
                    if level[edge.to] <= ln || edge.cap <= edge.flow {
                        iter[n] = i + 1;
                        continue;
                    }
                    // println!("try {} -> {}, cap {}", n, edge.to, edge.cap - edge.flow);
                    let d = dfs(g, edge.to, t, level, iter, (flow - f).min(edge.cap - edge.flow));
                    if d <= 0 {
                        iter[n] = i + 1;
                        continue;
                    }
                    // println!("{} -> {}, d = {}, cap={}", n, edge.to, d, edge.cap - edge.flow);
                    f += d;
                    g.edges[g.graph[n][i]].flow += d;
                    g.edges[g.graph[n][i] ^ 1].flow -= d;
                    if f == flow {
                        break;
                    }
                    iter[n] = i + 1;
                }
                // level[n] = -1;
                f
            }
            iter.fill(0);
            let f = dfs(self, s, t, &mut level, &mut iter, 21000);
            // println!("f {f}");
            if f == 0 {
                break;
            }
            max_flow += f;
        }
        max_flow
    }
}
impl Solution {
    pub fn guard_castle(grid: Vec<String>) -> i32 {
        let grid = grid
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let len = grid[0].len();
        let mut g = Flow::new(len * 4 + 3);
        let id = |x: usize, y: usize, i: usize| x * 2 * len + y * 2 + i;
        let s = len * 4;
        let t = len * 4 + 1;
        let warp = len * 4 + 2;
        let inf = 21000;
        for i in 0..2 {
            for j in 0..len {
                if grid[i][j] == '#' {
                    continue;
                }
                if grid[i][j] == 'S' {
                    g.add_edge(s, id(i, j, 1), inf);
                }
                if grid[i][j] == 'C' {
                    g.add_edge(id(i, j, 0), t, inf);
                }
                if grid[i][j] == 'P' {
                    g.add_edge(id(i, j, 0), warp, inf);
                    g.add_edge(warp, id(i, j, 1), inf);
                }
                if grid[i][j] == '.' {
                    g.add_edge(id(i, j, 0), id(i, j, 1), 1);
                }
                if i == 0 && grid[i + 1][j] != '#' {
                    g.add_edge(id(i, j, 1), id(i + 1, j, 0), inf);
                }
                if i == 1 && grid[i - 1][j] != '#' {
                    g.add_edge(id(i, j, 1), id(i - 1, j, 0), inf);
                }
                if j < len - 1 && grid[i][j + 1] != '#' {
                    g.add_edge(id(i, j, 1), id(i, j + 1, 0), inf);
                }
                if j > 0 && grid[i][j - 1] != '#' {
                    g.add_edge(id(i, j, 1), id(i, j - 1, 0), inf);
                }
            }
        }
        let f = g.max_flow(s, t);
        if f >= inf {
            -1
        } else {
            f
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        assert_eq!(Solution::guard_castle(vec_str!["S.", ".C"]), 2);
        assert_eq!(Solution::guard_castle(vec_str!["S.C.P#P.", ".....#.S"]), 3);
        assert_eq!(Solution::guard_castle(vec_str!["SP#P..P#PC#.S", "..#P..P####.#"]), -1);
        assert_eq!(Solution::guard_castle(vec_str!["SP#.C.#PS", "P.#...#.P"]), 0);
        assert_eq!(Solution::guard_castle(vec_str!["CP.#.P.", "...S..S"]), 4);
    }
}
