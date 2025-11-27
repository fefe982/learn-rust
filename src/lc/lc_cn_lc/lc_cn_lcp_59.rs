// https://leetcode.cn/problems/NfY1m5/
// LCP 59. 搭桥过河
pub struct Solution;
impl Solution {
    pub fn build_bridge(_num: i32, wood: Vec<Vec<i32>>) -> i64 {
        let mut l = std::collections::BinaryHeap::new();
        let mut r = std::collections::BinaryHeap::new();
        l.push(wood[0][0] as i64);
        r.push(std::cmp::Reverse(wood[0][0] as i64));
        let mut bl = 0i64;
        let mut br = 0i64;
        let mut ans = 0i64;
        for i in 1..wood.len() {
            bl -= (wood[i][1] - wood[i][0]) as i64;
            br += (wood[i - 1][1] - wood[i - 1][0]) as i64;
            let l0 = l.peek().unwrap() + bl;
            let r0 = r.peek().unwrap().0 + br;
            let wl = wood[i][0] as i64;
            if wl < l0 {
                ans += l0 - wl;
                l.pop();
                l.push(wl - bl);
                l.push(wl - bl);
                r.push(std::cmp::Reverse(l0 - br));
            } else if wl > r0 {
                ans += wl - r0;
                r.pop();
                r.push(std::cmp::Reverse(wl - br));
                r.push(std::cmp::Reverse(wl - br));
                l.push(r0 - bl);
            } else {
                l.push(wl - bl);
                r.push(std::cmp::Reverse(wl - br));
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
    fn build_bridge() {
        assert_eq!(Solution::build_bridge(10, vec_vec![[1, 2], [4, 7], [8, 9]]), 3);
        assert_eq!(
            Solution::build_bridge(10, vec_vec![[1, 5], [1, 1], [10, 10], [6, 7], [7, 8]]),
            10
        );
        assert_eq!(Solution::build_bridge(5, vec_vec![[1, 2], [2, 4]]), 0);
    }
}
