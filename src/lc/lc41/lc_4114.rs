// https://leetcode.com/problems/count-consistent-subarrays-ii/
// 4114. Count Consistent Subarrays II
pub struct Solution;
impl Solution {
    pub fn count_consistent(nums: Vec<i32>) -> i64 {
        let max_num = nums.iter().copied().max().unwrap_or(0) as usize;
        let mut positions = vec![0; max_num + 2];
        for &n in &nums {
            positions[n as usize + 1] += 1;
        }
        for i in 1..positions.len() {
            positions[i] += positions[i - 1];
        }
        let mut occurrences = vec![0; nums.len()];
        for (i, &n) in nums.iter().enumerate().rev() {
            let slot = n as usize + 1;
            positions[slot] -= 1;
            occurrences[positions[slot]] = i;
        }

        let mut cnt = 0;
        let mut v = Vec::with_capacity(20);
        let mut nxt: Vec<(i32, usize)> = Vec::with_capacity(20);
        for (i, &n) in nums.iter().enumerate() {
            v.push((n, i));
            nxt.clear();
            for &(av, ai) in &v {
                let nav = av & n;
                if nxt.is_empty() || nxt.last().unwrap().0 != nav {
                    nxt.push((nav, ai));
                }
            }
            std::mem::swap(&mut v, &mut nxt);
            v.push((-1, i + 1));
            for j in 0..v.len() - 1 {
                let x = v[j].0;
                let l = v[j].1;
                let r = v[j + 1].1;
                let start = positions[x as usize + 1];
                let end = if x as usize == max_num {
                    occurrences.len()
                } else {
                    positions[x as usize + 2]
                };
                let range = &occurrences[start..end];
                let ix = range.partition_point(|&p| p < r);
                if ix > 0 && range[ix - 1] >= l {
                    cnt += (range[ix - 1] - l + 1) as i64;
                }
            }
            v.pop();
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_consistent() {
        assert_eq!(Solution::count_consistent(vec![5, 3, 1, 2]), 6);
        assert_eq!(Solution::count_consistent(vec![1, 1, 1]), 6);
        assert_eq!(Solution::count_consistent(vec![0]), 1);
    }
}
