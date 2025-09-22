// https://leetcode.com/problems/jump-game-iii/
// 1306. Jump Game III
pub struct Solution;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut arr = arr;
        let start = start as usize;
        if arr[start] == 0 {
            return true;
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back(start);
        arr[start] = -arr[start];
        while let Some(i) = q.pop_front() {
            let d = (-arr[i]) as usize;
            if i >= d {
                if arr[i - d] == 0 {
                    return true;
                } else if arr[i - d] > 0 {
                    arr[i - d] = -arr[i - d];
                    q.push_back(i - d);
                }
            }
            if i + d < arr.len() {
                if arr[i + d] == 0 {
                    return true;
                } else if arr[i + d] > 0 {
                    arr[i + d] = -arr[i + d];
                    q.push_back(i + d);
                }
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_reach() {
        assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
        assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
        assert_eq!(Solution::can_reach(vec![3, 0, 2, 1, 2], 2), false);
    }
}
