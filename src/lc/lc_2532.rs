// https://leetcode.com/problems/time-to-cross-a-bridge/
// 2532. Time to Cross a Bridge
pub struct Solution;
impl Solution {
    pub fn find_crossing_time(mut n: i32, k: i32, mut time: Vec<Vec<i32>>) -> i32 {
        time.sort_by_key(|x| x[0] + x[2]);
        let mut t = 0;
        let mut left_heap = std::collections::BinaryHeap::new();
        let mut right_heap = left_heap.clone();
        for i in 0..time.len() {
            left_heap.push(i);
        }
        let mut left_work = std::collections::BinaryHeap::new();
        let mut right_work = left_work.clone();
        while n > 0 || !right_work.is_empty() || !right_heap.is_empty() {
            if let Some(w) = right_heap.pop() {
                t += time[w][2];
                left_work.push(std::cmp::Reverse((t + time[w][3], w)));
            } else if let (std::cmp::Ordering::Greater, Some(w)) = (n.cmp(&0), left_heap.pop()) {
                t += time[w][0];
                right_work.push(std::cmp::Reverse((t + time[w][1], w)));
                n -= 1;
            } else {
                t = left_work
                    .peek()
                    .unwrap_or(&std::cmp::Reverse((i32::MAX, 0)))
                    .0
                     .0
                    .min(
                        right_work
                            .peek()
                            .unwrap_or(&std::cmp::Reverse((i32::MAX, 0)))
                            .0
                             .0,
                    );
            }
            loop {
                if let Some(&std::cmp::Reverse((tw, w))) = left_work.peek() {
                    if tw <= t {
                        left_work.pop();
                        left_heap.push(w);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            loop {
                if let Some(&std::cmp::Reverse((tw, w))) = right_work.peek() {
                    if tw <= t {
                        right_work.pop();
                        right_heap.push(w);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_crossing_time() {
        assert_eq!(
            Solution::find_crossing_time(
                9,
                6,
                vec_vec![
                    [2, 6, 9, 4],
                    [4, 8, 7, 5],
                    [4, 6, 7, 6],
                    [2, 3, 3, 7],
                    [9, 3, 6, 8],
                    [2, 8, 8, 4]
                ]
            ),
            115
        );
        assert_eq!(
            Solution::find_crossing_time(
                14,
                7,
                vec_vec![
                    [8, 5, 9, 9],
                    [9, 10, 8, 8],
                    [9, 5, 4, 5],
                    [8, 8, 2, 2],
                    [8, 8, 9, 3],
                    [3, 6, 5, 6],
                    [4, 5, 6, 2]
                ]
            ),
            239
        );
        assert_eq!(
            Solution::find_crossing_time(1, 3, vec_vec![[1, 1, 2, 1], [1, 1, 3, 1], [1, 1, 4, 1]]),
            6
        );
        assert_eq!(
            Solution::find_crossing_time(3, 2, vec_vec![[1, 9, 1, 8], [10, 10, 10, 10]]),
            50
        );
    }
}
