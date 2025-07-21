// https://leetcode.com/problems/delivering-boxes-from-storage-to-ports/
// 1687. Delivering Boxes from Storage to Ports
pub struct Solution;
impl Solution {
    pub fn box_delivering(boxes: Vec<Vec<i32>>, _ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
        let mut ntrips = 2;
        let mut min_plus = std::collections::BinaryHeap::new();
        min_plus.push(std::cmp::Reverse((0, 0)));
        let mut sum_weight = 0;
        let mut weight_s = 0;
        for i in 0..boxes.len() {
            sum_weight += boxes[i][1];
            while sum_weight > max_weight {
                sum_weight -= boxes[weight_s][1];
                weight_s += 1;
            }
            let s = (i as i32 - max_boxes + 1).max(weight_s as i32) as usize;
            while let Some(&std::cmp::Reverse((plus, idx))) = min_plus.peek() {
                if idx < s {
                    min_plus.pop();
                } else {
                    if i == boxes.len() - 1 {
                        return ntrips + plus;
                    }
                    if boxes[i][0] == boxes[i + 1][0] {
                        min_plus.push(std::cmp::Reverse((plus + 2, i + 1)));
                    } else {
                        min_plus.push(std::cmp::Reverse((plus + 1, i + 1)));
                        ntrips += 1;
                    }
                    break;
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_box_delivering() {
        assert_eq!(Solution::box_delivering(vec_vec![[1, 1], [2, 1], [1, 1]], 2, 3, 3), 4);
        assert_eq!(
            Solution::box_delivering(vec_vec![[1, 2], [3, 3], [3, 1], [3, 1], [2, 4]], 3, 3, 6),
            6
        );
        assert_eq!(
            Solution::box_delivering(vec_vec![[1, 4], [1, 2], [2, 1], [2, 1], [3, 2], [3, 4]], 3, 6, 7),
            6
        );
    }
}
