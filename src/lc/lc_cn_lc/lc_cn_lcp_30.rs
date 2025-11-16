// https://leetcode.cn/problems/p0NxJO/
// LCP 30. 魔塔游戏
pub struct Solution;
impl Solution {
    pub fn magic_tower(nums: Vec<i32>) -> i32 {
        let mut sum = 1;
        let mut back = 0;
        let mut cnt = 0;
        let mut h = std::collections::BinaryHeap::<i32>::new();
        for n in nums {
            if n < 0 {
                h.push(-n);
            }
            sum = sum + n as i64;
            while sum <= 0 {
                if let Some(v) = h.pop() {
                    back += v as i64;
                    sum += v as i64;
                    cnt += 1;
                } else {
                    return -1;
                }
            }
        }
        if sum - back > 0 {
            cnt
        } else {
            -1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_magic_tower() {
        assert_eq!(
            Solution::magic_tower(vec![100, 100, 100, -250, -60, -140, -50, -50, 100, 150]),
            1
        );
        assert_eq!(Solution::magic_tower(vec![-200, -300, 400, 0]), -1);
    }
}
