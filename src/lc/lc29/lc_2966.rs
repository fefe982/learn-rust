// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/
// 2966. Divide Array Into Arrays With Max Difference
pub struct Solution;
impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..nums.len() / 3 {
            if nums[i * 3 + 2] - nums[i * 3] > k {
                return vec![];
            }
            ans.push(vec![nums[i * 3], nums[i * 3 + 1], nums[i * 3 + 2]]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(nums: Vec<i32>, k: i32, ans: Vec<Vec<i32>>, dividable: bool) -> bool {
        if dividable {
            let mut collect = std::collections::HashMap::<i32, i32>::new();
            for n in nums {
                *collect.entry(n).or_default() += 1;
            }
            for a in ans {
                let mut min = i32::MAX;
                let mut max = i32::MIN;
                for aa in a {
                    let c = collect.entry(aa).or_default();
                    *c -= 1;
                    if *c < 0 {
                        return false;
                    }
                    min = min.min(*c);
                    max = max.max(*c);
                    if max - min > k {
                        return false;
                    }
                }
            }
            return true;
        } else {
            return ans.is_empty();
        }
    }
    #[test]
    fn test_divide_array() {
        for (n, k, dividable) in [
            (vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2, true),
            (vec![1, 3, 3, 2, 7, 3], 3, false),
        ] {
            assert!(check(n.clone(), k, Solution::divide_array(n, k), dividable));
        }
    }
}
