// https://leetcode.com/problems/jump-game-v/
// 1340. Jump Game V
pub struct Solution;
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut left = vec![0; arr.len()];
        let mut right = vec![0; arr.len()];
        let mut v: Vec<(i32, usize)> = vec![];
        let mut h = std::collections::BinaryHeap::new();
        for (i, &n) in arr.iter().enumerate() {
            while let Some(&(ln, _)) = v.last() {
                if ln < n {
                    v.pop();
                } else {
                    break;
                }
            }
            if v.is_empty() {
                left[i] = 0;
            } else {
                left[i] = v.last().unwrap().1 + 1;
            }
            v.push((n, i));
            h.push(std::cmp::Reverse((n, i)))
        }
        v.clear();
        for (i, &n) in arr.iter().enumerate().rev() {
            while let Some(&(ln, _)) = v.last() {
                if ln < n {
                    v.pop();
                } else {
                    break;
                }
            }
            if v.is_empty() {
                right[i] = arr.len() - 1;
            } else {
                right[i] = v.last().unwrap().1 - 1;
            }
            v.push((n, i));
        }
        let mut ans = 0;
        let d = d as usize;
        let mut j = vec![0; arr.len()];
        while let Some(std::cmp::Reverse((_, i))) = h.pop() {
            let l = (left[i] + d).max(i) - d;
            let r = right[i].min(i + d);
            let jump = j[l..=r].iter().max().unwrap() + 1;
            ans = ans.max(jump);
            j[i] = jump;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_jumps() {
        assert_eq!(Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2), 4);
        assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
        assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    }
}
