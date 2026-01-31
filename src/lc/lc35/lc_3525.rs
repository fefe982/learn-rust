// https://leetcode.com/problems/find-x-value-of-array-ii/
// 3525. Find X Value of Array II
pub struct Solution;
impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        if k == 1 {
            return queries.into_iter().map(|x| nums.len() as i32 - x[2]).collect();
        }
        let len = nums.len();
        let n = 2 << (usize::BITS - (len - 1).leading_zeros());
        let mut tree = vec![(0, vec![0; k as usize]); n];
        for i in 0..len {
            let r = nums[i] % k;
            tree[i + n / 2 - 1].0 = r;
            tree[i + n / 2 - 1].1[r as usize] = 1;
        }
        fn merge(tree: &mut Vec<(i32, Vec<i32>)>, i: usize, k: i32) {
            let prod = tree[2 * i + 1].0;
            tree[i].0 = (prod * tree[2 * i + 2].0) % k;
            for j in 0..k as usize {
                tree[i].1[j] = tree[2 * i + 1].1[j]
            }
            for j in 0..k as usize {
                tree[i].1[(prod * j as i32 % k) as usize] += tree[2 * i + 2].1[j];
            }
        }
        for i in (0..n / 2 - 1).rev() {
            merge(&mut tree, i, k);
        }
        fn insert(tree: &mut Vec<(i32, Vec<i32>)>, idx: usize, l: usize, r: usize, i: usize, v: i32, k: i32) {
            if l + 1 == r {
                tree[idx].0 = v;
                tree[idx].1.fill(0);
                tree[idx].1[v as usize] = 1;
                return;
            }
            let mid = (l + r) / 2;
            if i < mid {
                insert(tree, 2 * idx + 1, l, mid, i, v, k);
            } else {
                insert(tree, 2 * idx + 2, mid, r, i, v, k);
            }
            merge(tree, idx, k);
        }
        fn query(tree: &Vec<(i32, Vec<i32>)>, idx: usize, l: usize, r: usize, q: usize, k: i32) -> (i32, Vec<i32>) {
            if q <= l {
                return tree[idx].clone();
            }
            let mid = (l + r) / 2;
            if q >= mid {
                return query(tree, 2 * idx + 2, mid, r, q, k);
            }
            let (lprod, mut lvec) = query(tree, 2 * idx + 1, l, mid, q, k);
            let (rprod, rvec) = query(tree, 2 * idx + 2, mid, r, q, k);
            let prod = (lprod * rprod) % k;
            for j in 0..k as usize {
                lvec[(lprod * j as i32 % k) as usize] += rvec[j];
            }
            (prod, lvec)
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            insert(&mut tree, 0, 0, n / 2, q[0] as usize, q[1] % k, k);
            ans.push(query(&tree, 0, 0, n / 2, q[2] as usize, k).1[q[3] as usize]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn result_array() {
        assert_eq!(
            Solution::result_array(
                vec![6, 9, 6, 7, 7, 9],
                5,
                vec_vec![[5, 46, 0, 1], [4, 1, 3, 3], [0, 33, 1, 1]]
            ),
            [3, 0, 0]
        );
        assert_eq!(
            Solution::result_array(
                vec![1, 2, 3, 4, 5],
                3,
                vec_vec![[2, 2, 0, 2], [3, 3, 3, 0], [0, 1, 0, 1]]
            ),
            [2, 2, 2]
        );
        assert_eq!(
            Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4, vec_vec![[0, 2, 0, 2], [0, 2, 0, 1]]),
            [1, 0]
        );
        assert_eq!(
            Solution::result_array(vec![1, 1, 2, 1, 1], 2, vec_vec![[2, 1, 0, 1]]),
            [5]
        );
    }
}
