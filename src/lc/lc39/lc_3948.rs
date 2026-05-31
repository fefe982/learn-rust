// https://leetcode.com/problems/lexicographically-maximum-mex-array/
// 3948. Lexicographically Maximum MEX Array
pub struct Solution;
impl Solution {
    pub fn maximum_mex(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        // Counts for values in [0, n]; values larger than n never affect the MEX.
        let mut remaining = vec![0usize; n + 1];
        for &x in &nums {
            let x = x as usize;
            if x <= n {
                remaining[x] += 1;
            }
        }

        // The suffix MEX only moves downward as we remove elements from the front.
        let mut mex = 0usize;
        while mex <= n && remaining[mex] > 0 {
            mex += 1;
        }

        let mut ans = Vec::new();
        let mut start = 0usize;

        while start < n {
            // When mex = 0, every prefix has mex 0, so taking one element keeps the prefix equal
            // and makes the final array longer, which is lexicographically better.
            if mex == 0 {
                ans.push(0);
                start += 1;
                continue;
            }

            // Shortest prefix that contains every value in [0, mex - 1].
            let mut seen = vec![false; mex];
            let mut need = mex;
            let mut end = start;

            while end < n {
                let x = nums[end] as usize;
                if x < mex && !seen[x] {
                    seen[x] = true;
                    need -= 1;
                    if need == 0 {
                        break;
                    }
                }
                end += 1;
            }

            ans.push(mex as i32);

            for idx in start..=end {
                let x = nums[idx] as usize;
                if x <= n {
                    remaining[x] -= 1;
                    if remaining[x] == 0 && x < mex {
                        mex = x;
                    }
                }
            }

            start = end + 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_mex() {
        assert_eq!(Solution::maximum_mex(vec![0, 1, 0]), vec![2, 1]);
        assert_eq!(Solution::maximum_mex(vec![3, 1]), vec![0, 0]);
        assert_eq!(Solution::maximum_mex(vec![0, 1, 2]), vec![3]);
        assert_eq!(Solution::maximum_mex(vec![1, 2, 3]), vec![0, 0, 0]);
        assert_eq!(Solution::maximum_mex(vec![1, 0, 1, 0]), vec![2, 2]);
        assert_eq!(Solution::maximum_mex(vec![0, 2, 1, 0, 1]), vec![3, 2]);
    }
}
