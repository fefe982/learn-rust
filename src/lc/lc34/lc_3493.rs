// https://leetcode.com/problems/properties-graph/
// 3493. Properties Graph
pub struct Solution;
impl Solution {
    fn parent(set: &mut Vec<usize>, x: usize) -> usize {
        if set[x] != x {
            set[x] = Solution::parent(set, set[x]);
        }
        set[x]
    }
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = properties.len();
        let mut p = vec![std::collections::HashSet::new(); n];
        for (i, prop) in properties.into_iter().enumerate() {
            for x in prop {
                p[i].insert(x);
            }
        }
        let mut set = (0..n).collect::<Vec<_>>();
        let mut res = n as i32;
        for i in 0..n {
            let pi = Solution::parent(&mut set, i);
            for j in i + 1..n {
                let pj = Solution::parent(&mut set, j);
                if pi == pj {
                    continue;
                }
                let c = p[i].intersection(&p[j]).count() as i32;
                if c >= k {
                    set[pj] = pi;
                    res -= 1;
                }
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
    fn number_of_components() {
        assert_eq!(
            Solution::number_of_components(vec_vec![[1, 2], [1, 1], [3, 4], [4, 5], [5, 6], [7, 7]], 1),
            3
        );
        assert_eq!(
            Solution::number_of_components(vec_vec![[1, 2, 3], [2, 3, 4], [4, 3, 5]], 2),
            1
        );
        assert_eq!(Solution::number_of_components(vec_vec![[1, 1], [1, 1]], 2), 2);
    }
}
