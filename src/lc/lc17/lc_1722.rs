// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/
// 1722. Minimize Hamming Distance After Swap Operations
pub struct Solution;
impl Solution {
    fn find(p: &mut Vec<usize>, x: usize) -> usize {
        if p[x] != x {
            p[x] = Self::find(p, p[x]);
        }
        p[x]
    }
    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let n = source.len();
        let mut p = (0..n).collect::<Vec<_>>();
        for v in allowed_swaps {
            let (a, b) = (v[0] as usize, v[1] as usize);
            let pa = Self::find(&mut p, a);
            let pb = Self::find(&mut p, b);
            p[pa] = pb;
        }
        let mut cnt = std::collections::HashMap::<usize, std::collections::HashMap<i32, i32>>::new();
        for i in 0..n {
            let pi = Self::find(&mut p, i);
            *cnt.entry(pi).or_default().entry(source[i]).or_default() += 1;
        }
        let mut ans = 0;
        for i in 0..n {
            let pi = Self::find(&mut p, i);
            if let Some(v) = cnt.get_mut(&pi) {
                if let Some(c) = v.get_mut(&target[i]) {
                    if *c > 0 {
                        *c -= 1;
                    } else {
                        ans += 1;
                    }
                } else {
                    ans += 1;
                }
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
    fn minimum_hamming_distance() {
        assert_eq!(
            Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![2, 1, 4, 5], vec_vec![[0, 1], [2, 3]]),
            1
        );
        assert_eq!(
            Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec_vec![]),
            2
        );
        assert_eq!(
            Solution::minimum_hamming_distance(
                vec![5, 1, 2, 4, 3],
                vec![1, 5, 4, 2, 3],
                vec_vec![[0, 4], [4, 2], [1, 3], [1, 4]]
            ),
            0
        )
    }
}
