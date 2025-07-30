// https://leetcode.com/problems/design-graph-with-shortest-path-calculator/
// 2642. Design Graph With Shortest Path Calculator
pub struct Graph {
    n: usize,
    g: std::collections::HashMap<i32, Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    pub fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut g = std::collections::HashMap::<i32, Vec<(i32, i32)>>::new();
        for edge in edges {
            g.entry(edge[0]).or_default().push((edge[1], edge[2]));
        }
        Self { n: n as usize, g }
    }

    pub fn add_edge(&mut self, edge: Vec<i32>) {
        self.g.entry(edge[0]).or_default().push((edge[1], edge[2]));
    }

    pub fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut q = std::collections::BinaryHeap::new();
        q.push((std::cmp::Reverse(0), node1));
        let mut visited = vec![false; self.n];
        while let Some((rd, n)) = q.pop() {
            let d = rd.0;
            if n == node2 {
                return d;
            }
            if visited[n as usize] {
                continue;
            }
            visited[n as usize] = true;
            if let Some(vn) = self.g.get(&n) {
                for &(node, dist) in vn {
                    q.push((std::cmp::Reverse(d + dist), node));
                }
            }
        }
        -1
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_graph() {
        let mut obj = Graph::new(4, vec_vec![[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
        assert_eq!(obj.shortest_path(3, 2), 6);
        assert_eq!(obj.shortest_path(0, 3), -1);
        obj.add_edge(vec![1, 3, 4]);
        assert_eq!(obj.shortest_path(0, 3), 6);
    }
}
