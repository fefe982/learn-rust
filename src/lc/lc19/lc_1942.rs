// https://leetcode.com/problems/the-number-of-the-smallest-unoccupied-chair/
// 1942. The Number of the Smallest Unoccupied Chair
pub struct Solution;
impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut times = times.into_iter().map(|x| (x[0], x[1])).zip(0..).collect::<Vec<_>>();
        times.sort();
        let mut free = std::collections::BinaryHeap::new();
        let mut leave = std::collections::BinaryHeap::new();
        let mut next = 0;
        for ((arrive, leaving), id) in times {
            while let Some(&(std::cmp::Reverse(time), chair)) = leave.peek() {
                if time <= arrive {
                    free.push(std::cmp::Reverse(chair));
                    leave.pop();
                } else {
                    break;
                }
            }
            if let Some(std::cmp::Reverse(chair)) = free.pop() {
                if id == target_friend {
                    return chair;
                }
                leave.push((std::cmp::Reverse(leaving), chair));
            } else {
                if id == target_friend {
                    return next;
                }
                leave.push((std::cmp::Reverse(leaving), next));
                next += 1;
            }
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_smallest_chair() {
        assert_eq!(Solution::smallest_chair(vec_vec![[1, 4], [2, 3], [4, 6]], 1), 1);
        assert_eq!(Solution::smallest_chair(vec_vec![[3, 10], [1, 5], [2, 6]], 0), 2);
    }
}
