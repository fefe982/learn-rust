// https://leetcode.com/problems/ways-to-make-a-fair-array/
// 1664. Ways to Make a Fair Array
pub struct Solution;
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let nodd = n / 2;
        let neven = n - nodd;
        let mut sodd = Vec::with_capacity(nodd + 1);
        let mut seven = Vec::with_capacity(neven + 1);
        sodd.push(0);
        seven.push(0);
        for i in 0..n {
            if i % 2 == 0 {
                seven.push(seven[i / 2] + nums[i]);
            } else {
                sodd.push(sodd[i / 2] + nums[i]);
            }
        }
        let mut ans = 0;
        for i in 0..n {
            let odd = sodd[i / 2] + seven[neven] - seven[(i + 2) / 2];
            let even = seven[(i + 1) / 2] + sodd[nodd] - sodd[(i + 1) / 2];
            if odd == even {
                ans += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ways_to_make_fair() {
        assert_eq!(Solution::ways_to_make_fair(vec![2, 1, 6, 4]), 1);
        assert_eq!(Solution::ways_to_make_fair(vec![1, 1, 1]), 3);
        assert_eq!(Solution::ways_to_make_fair(vec![1, 2, 3]), 0);
    }
}
