// https://leetcode.cn/problems/number-of-integers-with-popcount-depth-equal-to-k-ii/
// 3624. Number of Integers With Popcount Depth Equal to K II
pub struct Solution;
impl Solution {
    pub fn popcount_depth(nums: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i32> {
        let depth = |mut x: i64| -> usize {
            let mut d = 0;
            while x > 1 {
                x = x.count_ones() as i64;
                d += 1;
            }
            d
        };
        let mut nums = nums;
        let mut seg = vec![vec![0; nums.len() + 1]; 6];
        fn add(v: &mut Vec<i32>, idx: usize, val: i32) {
            let mut idx = idx;
            while idx < v.len() {
                v[idx] += val;
                idx += idx & (!idx + 1);
            }
        }
        fn get(v: &Vec<i32>, idx: usize) -> i32 {
            let mut idx = idx;
            let mut sum = 0;
            while idx > 0 {
                sum += v[idx];
                idx -= idx & (!idx + 1);
            }
            sum
        }
        for (&n, i) in nums.iter().zip(1..) {
            let d = depth(n);
            add(&mut seg[d], i, 1);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            if q[0] == 1 {
                let d = q[3] as usize;
                let c = get(&seg[d], q[2] as usize + 1) - get(&seg[d], q[1] as usize);
                ans.push(c);
            } else {
                let i = q[1] as usize + 1;
                let old_d = depth(nums[i - 1]);
                nums[i - 1] = q[2];
                let new_d = depth(nums[i - 1]);
                if old_d != new_d {
                    add(&mut seg[old_d], i, -1);
                    add(&mut seg[new_d], i, 1);
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
    fn popcount_depth() {
        assert_eq!(
            Solution::popcount_depth(vec![2, 4], vec_vec![[1, 0, 1, 1], [2, 1, 1], [1, 0, 1, 0]]),
            vec![2, 1]
        );
        assert_eq!(
            Solution::popcount_depth(
                vec![3, 5, 6],
                vec_vec![[1, 0, 2, 2], [2, 1, 4], [1, 1, 2, 1], [1, 0, 1, 0]]
            ),
            vec![3, 1, 0]
        );
        assert_eq!(
            Solution::popcount_depth(
                vec![1, 2],
                vec_vec![[1, 0, 1, 1], [2, 0, 3], [1, 0, 0, 1], [1, 0, 0, 2]]
            ),
            vec![1, 0, 1]
        );
    }
}
