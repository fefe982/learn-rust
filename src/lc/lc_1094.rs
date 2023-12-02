// https://leetcode.com/problems/car-pooling/
// 1094. Car Pooling
pub struct Solution;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut trips = trips;
        trips.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut cur = 0;
        let mut q = std::collections::BinaryHeap::new();
        for trip in trips {
            cur += trip[0];
            q.push((std::cmp::Reverse(trip[2]), trip[0]));
            while let Some((d, cap)) = q.peek() {
                if d.0 <= trip[1] {
                    cur -= cap;
                    q.pop();
                } else {
                    break;
                }
            }
            if cur > capacity {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_car_pooling() {
        assert_eq!(Solution::car_pooling(vec_vec![[2, 1, 5], [3, 3, 7]], 4), false);
        assert_eq!(Solution::car_pooling(vec_vec![[2, 1, 5], [3, 3, 7]], 5), true);
    }
}
