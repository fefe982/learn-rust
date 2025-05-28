// https://leetcode.com/problems/beautiful-arrangement-ii/
// 667. Beautiful Arrangement II
pub struct Solution;
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut i = 1;
        let mut j = n;
        let mut res = Vec::with_capacity(n as usize);
        let mut k = k;
        while k > 1 {
            if k % 2 == 1 {
                res.push(i);
                i += 1;
            } else {
                res.push(j);
                j -= 1;
            }
            k -= 1;
        }
        while i <= j {
            res.push(i);
            i += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(n: i32, k: i32) {
        let res = Solution::construct_array(n, k);
        let mut v = vec![0; n as usize];
        let mut cnt = 0;
        for i in 1..res.len() {
            let d = (res[i] - res[i - 1]).abs() as usize;
            if v[d] == 0 {
                cnt += 1;
            }
            v[d] = 1;
        }
        assert_eq!(cnt, k);
        assert_sort_eq!(res, (1..=n).collect::<Vec<i32>>());
    }
    #[test]
    fn construct_array() {
        check(3, 1);
        check(3, 2);
    }
}
