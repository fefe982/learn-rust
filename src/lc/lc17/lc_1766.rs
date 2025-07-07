// https://leetcode.com/problems/tree-of-coprimes/
// 1766. Tree of Coprimes
pub struct Solution;
impl Solution {
    fn traverse(
        nums: &Vec<i32>,
        coprimes: &Vec<Vec<usize>>,
        g: &Vec<std::collections::HashSet<usize>>,
        lvl: i32,
        p: usize,
        n: usize,
        path: &mut Vec<Vec<(i32, i32)>>,
        ans: &mut Vec<i32>,
    ) {
        let mut npr = -1;
        let mut nlvl = 0;
        if nums[n] == 1 {
            ans[n] = if p == usize::MAX { -1 } else { p as i32 };
        } else {
            for &cp in &coprimes[nums[n] as usize] {
                if let Some(&(idxn, lvln)) = path[cp].last() {
                    if lvln > nlvl {
                        npr = idxn;
                        nlvl = lvln;
                    }
                }
            }
            ans[n] = npr;
        }
        path[nums[n] as usize].push((n as i32, lvl));
        for &next in g[n].iter() {
            if next == p {
                continue;
            }
            Self::traverse(nums, coprimes, g, lvl + 1, n, next, path, ans);
        }
        path[nums[n] as usize].pop();
    }
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        let mut factor = vec![0; 51];
        for i in 2..=50 {
            let mut val = 0;
            let mut j = i;
            for (idx, &p) in primes.iter().enumerate() {
                if j % p == 0 {
                    val |= 1 << idx;
                    while j % p == 0 {
                        j /= p;
                    }
                }
                if j == 1 {
                    break;
                }
            }
            factor[i] = val;
        }
        let mut coprimes = vec![vec![]; 51];
        coprimes[1].push(1);
        for i in 1..=50 {
            for j in 1..i {
                if factor[i] & factor[j] == 0 {
                    coprimes[i].push(j);
                    coprimes[j].push(i);
                }
            }
        }
        let mut g = vec![std::collections::HashSet::new(); nums.len()];
        for e in edges {
            g[e[0] as usize].insert(e[1] as usize);
            g[e[1] as usize].insert(e[0] as usize);
        }
        let mut ans = vec![0; nums.len()];
        Self::traverse(&nums, &coprimes, &g, 1, usize::MAX, 0, &mut vec![vec![]; 51], &mut ans);
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_coprimes() {
        assert_eq!(
            Solution::get_coprimes(vec![2, 3, 3, 2], vec_vec![[0, 1], [1, 2], [1, 3]]),
            [-1, 0, 0, 1]
        );
        assert_eq!(
            Solution::get_coprimes(
                vec![5, 6, 10, 2, 3, 6, 15],
                vec_vec![[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]
            ),
            [-1, 0, -1, 0, 0, 0, -1]
        );
    }
}
