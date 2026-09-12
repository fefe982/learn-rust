// https://leetcode.com/problems/count-subarrays-with-distant-sums/
// 4051. Count Subarrays With Distant Sums
pub struct Solution;
impl Solution {
    pub fn distant_subarrays(nums: Vec<i32>, goal: i32, k: i32) -> i64 {
        let l = nums.len();
        let total = (l as i64) * (l as i64 + 1) / 2;
        if k == 0 {
            return total;
        }
        let mut m = std::collections::BTreeMap::new();
        let mut s = 0;
        m.insert(0, 0);
        for &n in &nums {
            s += n as i64;
            m.insert(s, 0);
        }
        let mut id = 1;
        for (_, v) in m.iter_mut() {
            *v = id;
            id += 1;
        }
        let mut tree = vec![0; m.len() + 1];
        let add = |tree: &mut Vec<i64>, i: usize, x: i64| {
            let mut i = i;
            while i < tree.len() {
                tree[i] += x;
                i += i & i.wrapping_neg();
            }
        };
        let query = |tree: &Vec<i64>, i: usize| {
            let mut res = 0;
            let mut i = i;
            while i > 0 {
                res += tree[i];
                i -= i & i.wrapping_neg();
            }
            res
        };
        add(&mut tree, m[&0] as usize, 1);
        let mut ans = 0;
        s = 0;
        let goal = goal as i64;
        let k = k as i64;
        for &n in &nums {
            s += n as i64;
            let l = *m.range(..=s - goal - k).rev().next().unwrap_or((&0, &0)).1;
            let r = *m.range(..s - goal + k).rev().next().unwrap_or((&0, &0)).1;
            if r > l {
                ans += query(&tree, r) - query(&tree, l);
            }
            add(&mut tree, m[&s] as usize, 1);
        }
        total - ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distant_subarrays() {
        assert_eq!(Solution::distant_subarrays(vec![1, 2, 1], 4, 1), 5);
        assert_eq!(Solution::distant_subarrays(vec![2, -1, 3], 2, 2), 2);
        assert_eq!(Solution::distant_subarrays(vec![-3, 1, 2], 0, 3), 2);
    }
}
