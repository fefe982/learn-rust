// https://leetcode.com/problems/make-sum-divisible-by-p/
// 1590. Make Sum Divisible by P
pub struct Solution;
use std::collections::HashMap;
impl Solution {
    fn insert(mod_map: &mut HashMap<i32, Vec<usize>>, n: i32, pos: usize) {
        if let Some(v) = mod_map.get_mut(&n) {
            v.push(pos);
        } else {
            mod_map.insert(n, vec![pos]);
        }
    }
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut mod_map: HashMap<i32, Vec<usize>> = HashMap::new();
        Self::insert(&mut mod_map, 0, 0);
        let mut modulo = 0;
        for (idx, v) in nums.iter().enumerate() {
            modulo = (modulo + *v) % p;
            Self::insert(&mut mod_map, modulo, idx + 1);
        }
        if modulo == 0 {
            return 0;
        }
        let mut min_step = i32::MAX;
        for (m, v1) in mod_map.iter() {
            if let Some(v2) = mod_map.get(&((*m + modulo) % p)) {
                let mut idx2 = 0;
                for pos1 in v1.iter() {
                    while idx2 < v2.len() && v2[idx2] < *pos1 {
                        idx2 += 1;
                    }
                    if idx2 >= v2.len() {
                        break;
                    }
                    if ((v2[idx2] - pos1) as i32) < min_step {
                        min_step = (v2[idx2] - pos1) as i32
                    }
                }
            }
        }
        if min_step == nums.len() as i32 {
            -1
        } else {
            min_step
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_subarray() {
        assert_eq!(Solution::min_subarray(vec![3, 1, 4, 2], 6), 1);
        assert_eq!(Solution::min_subarray(vec![6, 3, 5, 2], 9), 2);
        assert_eq!(Solution::min_subarray(vec![1, 2, 3], 3), 0);
    }
}
