// https://leetcode.com/problems/make-array-strictly-increasing/
// 1187. Make Array Strictly Increasing
use std::cmp;
use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
        arr2.sort_unstable();
        arr2.dedup();
        let arr2 = arr2;
        // problem constraint: max length is 2000.
        // max op is 2000, use a magic number larger than 2000
        let max = 10000;
        let mut keep_min_op = 0;
        let mut ex_min_op: HashMap<usize, i32> = HashMap::new();
        if arr2[0] < arr1[0] {
            ex_min_op.insert(0usize, 1);
        }
        for i in 1..arr1.len() {
            let mut current_keep_min_op = -1;
            let mut current_ex_min_op: HashMap<usize, i32> = HashMap::new();
            // from keep
            if keep_min_op >= 0 {
                // keep
                if arr1[i] > arr1[i - 1] {
                    current_keep_min_op = keep_min_op;
                }
                // exchange
                let exchange_idx = match arr2.binary_search(&arr1[i - 1]) {
                    Ok(idx) => idx + 1,
                    Err(idx) => idx,
                };
                if exchange_idx < arr2.len()
                    && (arr2[exchange_idx] < arr1[i] || current_keep_min_op < 0)
                {
                    current_ex_min_op.insert(exchange_idx, keep_min_op + 1);
                }
            }
            // from exchange
            for (&idx, &min_op) in ex_min_op.iter() {
                // keep
                if arr2[idx] < arr1[i] && (min_op < current_keep_min_op || current_keep_min_op < 0)
                {
                    current_keep_min_op = min_op;
                }
            }
            for (&idx, &min_op) in ex_min_op.iter() {
                // exchange
                if idx + 1 < arr2.len()
                    && (arr2[idx + 1] < arr1[i]
                        || current_keep_min_op < 0
                        || min_op + 1 < current_keep_min_op)
                {
                    current_ex_min_op.insert(
                        idx + 1,
                        cmp::min(
                            *current_ex_min_op.get(&(idx + 1)).unwrap_or(&max),
                            min_op + 1,
                        ),
                    );
                }
            }
            if current_keep_min_op < 0 && current_ex_min_op.len() == 0 {
                return -1;
            }
            keep_min_op = current_keep_min_op;
            ex_min_op = current_ex_min_op;
        }
        let mut min = keep_min_op;
        if min < 0 {
            min = max;
        }
        for &min_op in ex_min_op.values() {
            min = cmp::min(min, min_op)
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_array_increasing() {
        assert_eq!(
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]),
            1
        );
        assert_eq!(
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]),
            2
        );
        assert_eq!(
            Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]),
            -1
        );
    }
}
