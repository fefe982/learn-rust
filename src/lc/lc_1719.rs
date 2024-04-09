// https://leetcode.com/problems/number-of-ways-to-reconstruct-a-tree/
// 1719. Number Of Ways To Reconstruct A Tree
pub struct Solution;
impl Solution {
    fn get_p(v: &mut Vec<usize>, x: usize) -> usize {
        let mut px = x;
        while v[px] != px {
            px = v[px];
        }
        v[x] = px;
        px
    }
    fn ways(g: &mut std::collections::HashMap<i32, std::collections::HashSet<i32>>, node: Vec<i32>) -> i32 {
        if node.len() == 1 {
            return 1;
        }
        let mut root = 0;
        let mut nroot = 0;
        let nnode = node.len();
        let mut inode = std::collections::HashMap::new();
        for (i, &n) in node.iter().enumerate() {
            if g[&n].len() == nnode - 1 {
                nroot += 1;
                root = n;
            }
            inode.insert(n, i);
        }
        if nroot == 0 {
            return 0;
        }
        let mut v = (0..nnode).collect::<Vec<_>>();
        for &n in &node {
            if n == root {
                continue;
            }
            g.get_mut(&n).unwrap().remove(&root);
            for &adj in &g[&n] {
                let pa = Self::get_p(&mut v, inode[&adj]);
                let pn = Self::get_p(&mut v, inode[&n]);
                v[pa] = v[pn];
            }
        }
        let mut m = std::collections::HashMap::<usize, Option<Vec<i32>>>::new();
        for i in 0..nnode {
            if node[i] == root {
                continue;
            }
            m.entry(Self::get_p(&mut v, i))
                .or_insert(Some(vec![]))
                .as_mut()
                .unwrap()
                .push(node[i]);
        }
        let mut ways = 1;
        for (_, v) in m.iter_mut() {
            let nways_sub = Self::ways(g, v.take().unwrap());
            if nways_sub == 0 {
                return 0;
            } else if nways_sub == 2 {
                ways = 2;
            }
        }
        if ways == 2 || nroot > 1 {
            2
        } else {
            1
        }
    }
    pub fn check_ways(pairs: Vec<Vec<i32>>) -> i32 {
        let mut g = std::collections::HashMap::<i32, std::collections::HashSet<i32>>::new();
        for p in pairs {
            g.entry(p[0]).or_default().insert(p[1]);
            g.entry(p[1]).or_default().insert(p[0]);
        }
        let node = g.iter().map(|(k, _)| *k).collect::<Vec<_>>();
        Self::ways(&mut g, node)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_check_ways() {
        assert_eq!(Solution::check_ways(vec_vec![[1, 2], [2, 3]]), 1);
        assert_eq!(Solution::check_ways(vec_vec![[1, 2], [2, 3], [1, 3]]), 2);
        assert_eq!(Solution::check_ways(vec_vec![[1, 2], [2, 3], [2, 4], [1, 5]]), 0);
    }
}
