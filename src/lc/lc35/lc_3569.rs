// https://leetcode.com/problems/maximize-count-of-distinct-primes-after-split/
// 3569. Maximize Count of Distinct Primes After Split
pub struct Solution;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut is_prime = vec![true; 100001];
        is_prime[1] = false;
        for i in 2..316 {
            if !is_prime[i] {
                continue;
            }
            for j in 2..=100000 / i {
                is_prime[i * j] = false;
            }
        }
        let nt = 1 << (usize::BITS - nums.len().leading_zeros());
        let mut val = vec![0; nt * 2];
        let mut todo = vec![0; nt * 2];
        fn add(val: &mut Vec<i32>, todo: &mut Vec<i32>, i: usize, il: usize, ir: usize, l: usize, r: usize, v: i32) {
            if r <= il || ir <= l {
                return;
            }
            if l <= il && ir <= r {
                val[i] += v;
                todo[i] += v;
                return;
            }
            spread(val, todo, i, il, ir);
            let im = (il + ir) / 2;
            add(val, todo, 2 * i + 1, il, im, l, r, v);
            add(val, todo, 2 * i + 2, im, ir, l, r, v);
            val[i] = val[2 * i + 1].max(val[2 * i + 2]);
        }
        fn spread(val: &mut Vec<i32>, todo: &mut Vec<i32>, i: usize, il: usize, ir: usize) {
            if todo[i] == 0 {
                return;
            }
            let v = todo[i];
            todo[i] = 0;
            if il + 1 == ir {
                return;
            }
            val[2 * i + 1] += v;
            todo[2 * i + 1] += v;
            val[2 * i + 2] += v;
            todo[2 * i + 2] += v;
        }
        let mut mp = std::collections::HashMap::<i32, std::collections::BTreeSet<usize>>::new();
        for i in 0..nums.len() {
            if is_prime[nums[i] as usize] {
                mp.entry(nums[i]).or_default().insert(i);
            }
        }
        for (_, v) in mp.iter() {
            if v.len() > 1 {
                add(
                    &mut val,
                    &mut todo,
                    0,
                    0,
                    nt,
                    *v.first().unwrap(),
                    *v.last().unwrap(),
                    1,
                );
            }
        }
        let mut ans = Vec::with_capacity(queries.len());
        let mut nums = nums;
        for q in queries {
            let qi = q[0] as usize;
            let nv = nums[qi];
            let qv = q[1];
            if qv != nv {
                if is_prime[nv as usize] {
                    let niset = mp.entry(nv).or_default();
                    niset.remove(&qi);
                    if niset.len() > 0 {
                        if qi < *niset.first().unwrap() {
                            add(&mut val, &mut todo, 0, 0, nt, qi, *niset.first().unwrap(), -1);
                        }
                        if qi > *niset.last().unwrap() {
                            add(&mut val, &mut todo, 0, 0, nt, *niset.last().unwrap(), qi, -1);
                        }
                    } else {
                        mp.remove(&nv);
                    }
                }
                if is_prime[qv as usize] {
                    let qiset = mp.entry(qv).or_default();
                    if qiset.len() > 0 {
                        if qi < *qiset.first().unwrap() {
                            add(&mut val, &mut todo, 0, 0, nt, qi, *qiset.first().unwrap(), 1);
                        }
                        if qi > *qiset.last().unwrap() {
                            add(&mut val, &mut todo, 0, 0, nt, *qiset.last().unwrap(), qi, 1);
                        }
                    }
                    qiset.insert(qi);
                }
                nums[qi] = qv;
            }
            ans.push(val[0] + mp.len() as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn maximum_count() {
        assert_eq!(
            Solution::maximum_count(vec![2, 1, 3, 1, 2], vec_vec![[1, 2], [3, 3]]),
            [3, 4]
        );
        assert_eq!(Solution::maximum_count(vec![2, 1, 4], vec_vec![[0, 1]]), [0]);
    }
}
