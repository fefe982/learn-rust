// https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal/
// 3505. Minimum Operations to Make Elements Within K Subarrays Equal
pub struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let x = x as usize;
        let mut hl = std::collections::BinaryHeap::<(i64, usize)>::new();
        let mut hr = std::collections::BinaryHeap::<std::cmp::Reverse<(i64, usize)>>::new();
        let nr = x / 2;
        let nl = x - nr;
        let mut cr = 0;
        let mut cl = 0;
        let mut sr = 0;
        let mut sl = 0;
        let mut nop = std::collections::VecDeque::new();
        let k = k as usize;
        let mut cost_v = vec![i64::MAX / 2; k + 1];
        cost_v[0] = 0;
        nop.push_back(cost_v.clone());
        for i in 0..nums.len() {
            let mid = hl.peek().map(|x| x.0).unwrap_or(i64::MAX);
            if i >= x {
                let pop = nums[i - x] as i64;
                if pop <= mid {
                    sl -= pop;
                    cl -= 1;
                } else {
                    sr -= pop;
                    cr -= 1;
                }
            }
            let ins = nums[i] as i64;
            if ins < mid {
                hl.push((ins, i));
                sl += ins;
                cl += 1;
            } else {
                hr.push(std::cmp::Reverse((ins, i)));
                sr += ins;
                cr += 1;
            }
            while cl > nl {
                let pop = hl.pop().unwrap();
                if pop.1 + x > i {
                    sl -= pop.0;
                    cl -= 1;
                    hr.push(std::cmp::Reverse(pop));
                    sr += pop.0;
                    cr += 1;
                }
            }
            while cr > nr {
                let pop = hr.pop().unwrap().0;
                if pop.1 + x > i {
                    sr -= pop.0;
                    cr -= 1;
                    hl.push((pop.0, pop.1));
                    sl += pop.0;
                    cl += 1;
                }
            }
            while hl.peek().unwrap().1 + x <= i {
                hl.pop();
            }
            if i >= x - 1 {
                let mut cost = sr - sl;
                if nl > nr {
                    cost += hl.peek().unwrap().0;
                }
                let mut lv = nop.pop_front().unwrap();
                let prev = nop.back().unwrap();
                for i in (0..k).rev() {
                    lv[i + 1] = prev[i + 1].min(cost + lv[i]);
                }
                nop.push_back(lv);
            } else {
                nop.push_back(cost_v.clone());
            }
        }
        nop.back().unwrap()[k]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(Solution::min_operations(vec![3, 6, -5, -5, -17, 8], 2, 2), 3);
        assert_eq!(Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2), 8);
        assert_eq!(Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2), 3);
    }
}
