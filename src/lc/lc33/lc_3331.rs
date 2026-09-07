// https://leetcode.com/problems/find-subtree-sizes-after-changes/
// 3331. Find Subtree Sizes After Changes
pub struct Solution;
impl Solution {
    fn walk(
        tree: &Vec<Vec<usize>>,
        s: &[u8],
        node: usize,
        parent: usize,
        root: &mut [usize],
        count: &mut [i32],
    ) -> i32 {
        let idx = (s[node] - b'a') as usize;
        let root_save = root[idx];
        root[idx] = node;
        let mut total = 0;
        for &child in &tree[node] {
            if child != parent {
                total += Self::walk(tree, s, child, node, root, count);
            }
        }
        total += 1;
        root[idx] = root_save;
        count[node] += total;
        if root_save == usize::MAX {
            count[node]
        } else {
            count[root_save] += count[node];
            0
        }
    }
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let mut tree = vec![vec![]; n];
        for (i, &p) in parent.iter().enumerate() {
            if p != -1 {
                tree[p as usize].push(i);
            }
        }
        let s = s.as_bytes();
        let mut root = vec![usize::MAX; 26];
        let mut count = vec![0; n];
        Self::walk(&tree, s, 0, usize::MAX, &mut root, &mut count);
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_subtree_sizes() {
        assert_eq!(
            Solution::find_subtree_sizes(
                vec![-1, 10, 0, 12, 10, 18, 11, 12, 2, 3, 2, 2, 2, 0, 4, 11, 4, 2, 0],
                //    b    a  b  a   d   a   b   b  d  a  b  c  b  a  c  e   e  d  a
                "babadabbdabcbaceeda".to_string()
            ),
            vec![19, 1, 15, 2, 3, 1, 1, 1, 1, 1, 5, 2, 4, 1, 1, 1, 1, 1, 2]
        );
        assert_eq!(
            Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "abaabc".to_string()),
            vec![6, 3, 1, 1, 1, 1]
        );
        assert_eq!(
            Solution::find_subtree_sizes(vec![-1, 0, 4, 0, 1], "abbba".to_string()),
            vec![5, 2, 1, 1, 1]
        );
    }
}
