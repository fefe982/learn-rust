// https://leetcode.com/problems/minimum-incompatibility/
// 1681. Minimum Incompatibility
pub struct Solution;
impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        if k == nums.len() as i32 {
            return 0;
        }
        let tot: usize = 1 << nums.len();
        let bsz = nums.len() as i32 / k;
        let mut values = vec![i32::MAX; tot];
        for mask in 1..tot {
            if mask.count_ones() as i32 != bsz {
                continue;
            }
            let mut dup = vec![false; nums.len()];
            let mut mx = 0;
            let mut mn = i32::MAX;
            let mut cnt = 0;
            for i in 0..nums.len() as usize {
                if mask & (1 << i) == 0 {
                    continue;
                }
                if dup[nums[i] as usize - 1] {
                    break;
                }
                dup[nums[i] as usize - 1] = true;
                mx = mx.max(nums[i]);
                mn = mn.min(nums[i]);
                cnt += 1;
            }
            if cnt == bsz {
                values[mask] = mx - mn;
            }
        }
        for mask in 1..tot {
            if values[mask] == i32::MAX {
                continue;
            }
            let inv = tot - 1 - mask;
            let mut sub = inv;
            while sub != 0 {
                if values[sub] != i32::MAX {
                    values[mask | sub] = values[mask | sub].min(values[mask] + values[sub]);
                }
                sub = (sub - 1) & inv;
            }
        }
        if values[tot - 1] == i32::MAX {
            -1
        } else {
            values[tot - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_incompatibility() {
        assert_eq!(
            Solution::minimum_incompatibility(
                vec![5, 3, 4, 14, 5, 11, 10, 11, 14, 2, 10, 16, 9, 12, 2, 3],
                16
            ),
            0
        );
        assert_eq!(
            Solution::minimum_incompatibility(
                vec![7, 3, 16, 15, 1, 13, 1, 2, 14, 5, 3, 10, 6, 2, 7, 15],
                8
            ),
            12
        );
        assert_eq!(Solution::minimum_incompatibility(vec![1, 2, 1, 4], 2), 4);
        assert_eq!(
            Solution::minimum_incompatibility(vec![6, 3, 8, 1, 3, 1, 2, 2], 4),
            6
        );
        assert_eq!(
            Solution::minimum_incompatibility(vec![5, 3, 3, 6, 3, 3], 3),
            -1
        );
    }
}
