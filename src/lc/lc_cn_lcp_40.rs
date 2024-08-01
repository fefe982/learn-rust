// https://leetcode.cn/problems/uOAnQW/
// LCP 40. 心算挑战
pub struct Solution;
impl Solution {
    pub fn maxmium_score(cards: Vec<i32>, cnt: i32) -> i32 {
        let mut even = std::collections::BinaryHeap::new();
        let mut odd = std::collections::BinaryHeap::new();
        for c in cards {
            if c % 2 == 1 {
                odd.push(c);
            } else {
                even.push(c);
            }
        }
        let mut cnt = cnt;
        let mut ans = 0;
        if cnt % 2 == 1 {
            if let Some(e) = even.pop() {
                ans = e;
            } else {
                return 0;
            }
            cnt -= 1;
        }
        let mut top_even = 0;
        let mut top_odd = 0;
        while cnt > 0 {
            if !even.is_empty() && top_even == 0 {
                if let Some(e0) = even.pop() {
                    if let Some(e1) = even.pop() {
                        top_even = e0 + e1;
                    }
                }
            }
            if !odd.is_empty() && top_odd == 0 {
                if let Some(o0) = odd.pop() {
                    if let Some(o1) = odd.pop() {
                        top_odd = o0 + o1;
                    }
                }
            }
            if top_even == 0 && top_odd == 0 {
                return 0;
            }
            if top_even > top_odd {
                ans += top_even;
                cnt -= 2;
                top_even = 0;
            } else {
                ans += top_odd;
                cnt -= 2;
                top_odd = 0;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maxmium_score() {
        assert_eq!(Solution::maxmium_score(vec![1, 2, 8, 9], 3), 18);
        assert_eq!(Solution::maxmium_score(vec![3, 3, 1], 1), 0);
    }
}
