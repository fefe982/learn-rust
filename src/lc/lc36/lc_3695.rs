// https://leetcode.cn/problems/maximize-alternating-sum-using-swaps/
// 3695. Maximize Alternating Sum Using Swaps
pub struct Solution;
impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>, swaps: Vec<Vec<i32>>) -> i64 {
        let len = nums.len();
        let mut arr = (0..len)
            .map(|x| (x, (x % 2) as i32, (1 - x % 2) as i32))
            .collect::<Vec<(usize, i32, i32)>>();
        fn p(arr: &mut Vec<(usize, i32, i32)>, x: usize) -> usize {
            if arr[x].0 != x {
                arr[x].0 = p(arr, arr[x].0);
            }
            arr[x].0
        }
        fn u(arr: &mut Vec<(usize, i32, i32)>, x: usize, y: usize) {
            let px = p(arr, x);
            let py = p(arr, y);
            if px != py {
                arr[px].0 = py;
                arr[py].1 += arr[px].1;
                arr[py].2 += arr[px].2;
            }
        }
        for swap in swaps {
            u(&mut arr, swap[0] as usize, swap[1] as usize);
        }
        let mut sum = 0;
        let mut m = std::collections::HashMap::new();
        for i in 0..len {
            let p = p(&mut arr, i);
            let no = arr[p].1;
            let ne = arr[p].2;
            if no == 0 {
                sum += nums[i] as i64;
                continue;
            }
            if ne == 0 {
                sum -= nums[i] as i64;
                continue;
            }
            let entry = m.entry(p).or_insert(std::collections::BinaryHeap::new());
            let nn = no.min(ne) as usize;
            let mut n = nums[i];
            if no > ne {
                n = -n;
            }
            entry.push(n);
            if entry.len() > nn {
                sum += entry.pop().unwrap() as i64;
            }
        }
        for (_, heap) in m {
            sum -= heap.into_iter().map(|x| x as i64).sum::<i64>();
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_alternating_sum() {
        assert_eq!(
            Solution::max_alternating_sum(vec![1, 2, 3], vec_vec![[0, 2], [1, 2]]),
            4
        );
        assert_eq!(Solution::max_alternating_sum(vec![1, 2, 3], vec_vec![[1, 2]]), 2);
        assert_eq!(
            Solution::max_alternating_sum(vec![1, 1000000000, 1, 1000000000, 1, 1000000000], vec_vec![]),
            -2999999997
        );
    }
}
