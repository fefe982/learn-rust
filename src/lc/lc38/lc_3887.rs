// https://leetcode.com/problems/incremental-even-weighted-cycle-queries/
// 3887. Incremental Even Weighted Cycle Queries
pub struct Solution;
struct DsuParity {
    parent: Vec<usize>,
    size: Vec<usize>,
    // xor_to_parent[x] = parity from x to parent[x]
    xor_to_parent: Vec<i32>,
}

impl DsuParity {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            xor_to_parent: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> (usize, i32) {
        if self.parent[x] == x {
            return (x, 0);
        }
        let p = self.parent[x];
        let (root, xor_up) = self.find(p);
        self.parent[x] = root;
        self.xor_to_parent[x] ^= xor_up;
        (self.parent[x], self.xor_to_parent[x])
    }

    fn union_with_parity(&mut self, u: usize, v: usize, w: i32) -> bool {
        let (ru, xu) = self.find(u);
        let (rv, xv) = self.find(v);

        if ru == rv {
            return (xu ^ xv) == w;
        }

        // Need parity(ru -> rv) so that parity(u -> v) equals w.
        let link = xu ^ xv ^ w;
        if self.size[ru] < self.size[rv] {
            self.parent[ru] = rv;
            self.xor_to_parent[ru] = link;
            self.size[rv] += self.size[ru];
        } else {
            self.parent[rv] = ru;
            self.xor_to_parent[rv] = link;
            self.size[ru] += self.size[rv];
        }
        true
    }
}

impl Solution {
    pub fn number_of_edges_added(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut dsu = DsuParity::new(n as usize);
        let mut added = 0;
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            if dsu.union_with_parity(u, v, w) {
                added += 1;
            }
        }
        added
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn number_of_edges_added() {
        assert_eq!(
            Solution::number_of_edges_added(3, vec_vec![[0, 1, 1], [1, 2, 1], [0, 2, 1]]),
            2
        );
        assert_eq!(
            Solution::number_of_edges_added(3, vec_vec![[0, 1, 1], [1, 2, 1], [0, 2, 0]]),
            3
        );
    }
}
