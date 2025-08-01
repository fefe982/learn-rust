// https://leetcode.cn/problems/maximize-win-from-two-segments/
// 2555. Maximize Win From Two Segments
pub struct Solution;
impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let mut sum = std::collections::VecDeque::new();
        let mut ans = 0;
        let len = prize_positions.len();
        let mut ileft = 0;
        let mut iright = 0;
        let mut left = prize_positions[ileft];
        let mut right = prize_positions[iright];
        let mut max_left = 0;
        loop {
            while iright + 1 < len && prize_positions[iright + 1] - left <= k {
                iright += 1;
                if prize_positions[iright] != right {
                    sum.push_back((right, (iright - ileft) as i32));
                    right = prize_positions[iright];
                }
            }
            ans = ans.max((iright - ileft) as i32 + 1 + max_left);
            sum.push_back((right, (iright - ileft) as i32 + 1));
            if iright + 1 >= len {
                break;
            }
            iright += 1;
            right = prize_positions[iright];
            while right - prize_positions[ileft] > k {
                ileft += 1;
                left = prize_positions[ileft];
            }
            while let Some(&(pos, val)) = sum.front() {
                if pos < left {
                    max_left = max_left.max(val);
                    sum.pop_front();
                } else {
                    break;
                }
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximize_win() {
        assert_eq!(Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2), 7);
        assert_eq!(Solution::maximize_win(vec![1, 2, 3, 4], 0), 2);
    }
}
