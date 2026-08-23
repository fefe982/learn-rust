// https://leetcode.com/problems/valid-k-unique-subarrays-i/
// 4033. Valid K Unique Subarrays I
pub struct Solution;
impl Solution {
    pub fn valid_subarrays(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        use rand;
        let mut s = Vec::with_capacity(nums.len() + 1);
        let mut m = std::collections::HashMap::new();
        let mut ss = 0;
        s.push(ss);
        for &n in &nums {
            if let Some(v) = m.get(&n) {
                ss ^= v;
            } else {
                let v = rand::random::<i64>();
                m.insert(n, v);
                ss ^= v;
            }
            s.push(ss);
        }
        m.clear();
        let calc_left = |k: usize| -> Vec<usize> {
            let mut lefts = vec![0; nums.len()];
            let mut cnt = std::collections::HashMap::new();
            let mut l = 0;
            for (i, &n) in nums.iter().enumerate() {
                *cnt.entry(n).or_insert(0) += 1;
                while cnt.len() >= k {
                    let v = nums[l];
                    let &r = cnt.get(&v).unwrap();
                    if r > 1 {
                        cnt.insert(v, r - 1);
                    } else {
                        cnt.remove(&v);
                    }
                    l += 1;
                }
                lefts[i] = l;
            }
            lefts
        };
        let l1 = calc_left(k as usize);
        let l2 = calc_left(k as usize + 1);
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let lq = q[0] as usize;
            let rq = q[1] as usize;
            ans.push(s[rq + 1] == s[lq] && l2[rq] <= lq && lq < l1[rq])
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn valid_subarrays() {
        assert_eq!(
            Solution::valid_subarrays(vec![1, 2, 2, 1], 2, vec_vec![[0, 1], [0, 3], [1, 2]]),
            vec![false, true, false]
        );
        assert_eq!(
            Solution::valid_subarrays(vec![3, 3, 3], 1, vec_vec![[1, 2], [0, 2]]),
            vec![true, false]
        );
    }
}
