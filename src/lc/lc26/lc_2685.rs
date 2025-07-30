// https://leetcode.com/problems/count-the-number-of-complete-components
// 2685. Count the Number of Complete Components
pub struct Solution;
impl Solution {
    fn get_root(parents: &mut Vec<usize>, x: usize) -> usize {
        if parents[x] == x {
            return x;
        }
        let p = Self::get_root(parents, parents[x]);
        parents[x] = p;
        p
    }
    fn merge(parents: &mut Vec<usize>, x: usize, y: usize, counts: &mut Vec<(usize, usize)>) {
        let x = Self::get_root(parents, x);
        let y = Self::get_root(parents, y);
        if x != y {
            parents[x] = y;
            counts[y].0 += counts[x].0;
            counts[y].1 += counts[x].1 + 1;
        } else {
            counts[y].1 += 1;
        }
    }
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parents = (0..n).collect::<Vec<_>>();
        let mut counts = vec![(1, 0); n];
        for edge in edges {
            Self::merge(&mut parents, edge[0] as usize, edge[1] as usize, &mut counts);
        }
        let mut res = 0;
        for i in 0..n {
            if parents[i] == i && counts[i].1 == counts[i].0 * (counts[i].0 - 1) / 2 {
                res += 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_count_complete_components() {
        assert_eq!(
            Solution::count_complete_components(6, vec_vec![[0, 1], [0, 2], [1, 2], [3, 4]]),
            3
        );
        assert_eq!(
            Solution::count_complete_components(6, vec_vec![[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]]),
            1
        );
    }
}
