// https://leetcode.com/problems/threshold-majority-queries/
// 3636. Threshold Majority Queries
pub struct Solution;
impl Solution {
    pub fn subarray_majority(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = std::collections::BTreeMap::new();
        for &n in &nums {
            m.insert(n, 1);
        }
        for (i, (_, v)) in m.iter_mut().enumerate() {
            *v = i;
        }
        let mut vm = Vec::with_capacity(m.len());
        for (&k, _) in m.iter() {
            vm.push(k);
        }
        let mut nums = nums;
        let mut im = vec![vec![]; m.len()];
        for i in 0..nums.len() {
            let mi = m[&nums[i]];
            nums[i] = mi as i32;
            im[mi].push(i);
        }
        let sblk = nums.len().isqrt();
        let nblk = nums.len() / sblk;
        let mut min = vec![vec![0; nblk]; nblk];
        for i in 0..nblk {
            let mut cnt = vec![0; m.len()];
            let mut minval = 0;
            let mut maxcnt = 0;
            for j in i..nblk {
                for k in (j * sblk)..(j + 1) * sblk {
                    cnt[nums[k] as usize] += 1;
                    if cnt[nums[k] as usize] > maxcnt || (cnt[nums[k] as usize] == maxcnt && nums[k] < minval) {
                        maxcnt = cnt[nums[k] as usize];
                        minval = nums[k];
                    }
                }
                min[i][j] = minval;
            }
        }
        let mut ans = Vec::with_capacity(queries.len());
        let mut c = vec![0; m.len()];
        let mut z = 0;
        let query = |l: usize, r: usize, num: i32| {
            let v = &im[num as usize];
            v.partition_point(|&i| r > i) - v.partition_point(|&i| l > i)
        };
        let mut check = |num: i32, l: usize, r: usize, z: i32, minv: &mut i32, maxf: &mut usize| {
            if c[num as usize] == z {
                return;
            }
            c[num as usize] = z;
            let f = query(l, r, num);
            if f > *maxf || (f == *maxf && num < *minv) {
                *maxf = f;
                *minv = num;
            }
        };
        for q in queries {
            let (l, r, t) = (q[0] as usize, q[1] as usize + 1, q[2] as usize);
            let lb = (l + sblk - 1) / sblk;
            let rb = r / sblk;
            z += 1;
            let mut minv = 0;
            let mut maxf = 0;
            if rb > lb {
                for i in l..lb * sblk {
                    check(nums[i], l, r, z, &mut minv, &mut maxf);
                }
                for i in rb * sblk..r {
                    check(nums[i], l, r, z, &mut minv, &mut maxf);
                }
                check(min[lb][rb - 1], l, r, z, &mut minv, &mut maxf);
            } else {
                for i in l..r {
                    check(nums[i], l, r, z, &mut minv, &mut maxf);
                }
            }
            if maxf >= t {
                ans.push(vm[minv as usize]);
            } else {
                ans.push(-1);
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
    fn subarray_majority() {
        assert_eq!(
            Solution::subarray_majority(vec![1, 1, 2, 2, 1, 1], vec_vec![[0, 5, 4], [0, 3, 3], [2, 3, 2]]),
            vec![1, -1, 2]
        );
        assert_eq!(
            Solution::subarray_majority(
                vec![3, 2, 3, 2, 3, 2, 3],
                vec_vec![[0, 6, 4], [1, 5, 2], [2, 4, 1], [3, 3, 1]]
            ),
            vec![3, 2, 3, 2]
        );
    }
}
