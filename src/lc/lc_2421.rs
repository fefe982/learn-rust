// https://leetcode.com/problems/number-of-good-paths/
// 2421. Number of Good Paths
pub struct Solution;
impl Solution {
    fn find(union: &mut Vec<usize>, i: usize) -> usize {
        let mut p = i;
        while union[p] != p {
            p = union[p];
        }
        union[i] = p;
        p
    }
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }
        let mut vm = std::collections::BTreeMap::<i32, Vec<usize>>::new();
        for (i, &v) in vals.iter().enumerate() {
            vm.entry(v).or_default().push(i);
        }
        let mut ans = 0;
        let mut union = (0..n).collect::<Vec<_>>();
        for (v, nv) in vm {
            for &i in &nv {
                for &j in &g[i] {
                    if vals[j] <= v {
                        let pi = Self::find(&mut union, i);
                        let pj = Self::find(&mut union, j);
                        union[pj] = pi;
                    }
                }
            }
            let mut cnt = std::collections::HashMap::<usize, i32>::new();
            for i in nv {
                cnt.entry(Self::find(&mut union, i))
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            for c in cnt.values() {
                ans += c * (c + 1) / 2;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_of_good_paths() {
        assert_eq!(
            Solution::number_of_good_paths(vec![1, 3, 2, 1, 3], vec_vec![[0, 1], [0, 2], [2, 3], [2, 4]]),
            6
        );
        assert_eq!(
            Solution::number_of_good_paths(vec![1, 1, 2, 2, 3], vec_vec![[0, 1], [1, 2], [2, 3], [2, 4]]),
            7
        );
        assert_eq!(Solution::number_of_good_paths(vec![1], vec_vec![]), 1);
    }
}
