// https://leetcode.com/problems/maximum-subarray-xor-with-bounded-range/
// 3845. Maximum Subarray XOR With Bounded Range
pub struct Solution;
impl Solution {
    pub fn max_xor(nums: Vec<i32>, k: i32) -> i32 {
        let mut trie = vec![0; 1 << 16];
        let mut maxq = std::collections::VecDeque::new();
        let mut minq = std::collections::VecDeque::new();
        let mut s = 0;
        let mut xor = vec![0; nums.len() + 1];
        let mut idx = 1;
        for _ in (0..15).rev() {
            idx = 2 * idx;
            trie[idx] += 1;
        }
        let mut ans = 0;
        for (i, &n) in nums.iter().enumerate() {
            xor[i + 1] = xor[i] ^ n;
            idx = 1;
            for j in (0..15).rev() {
                let bit = (xor[i + 1] >> j) & 1;
                idx = 2 * idx + bit as usize;
                trie[idx] += 1;
            }
            while let Some(&(m, _)) = maxq.back() {
                if m < n {
                    maxq.pop_back();
                } else {
                    break;
                }
            }
            maxq.push_back((n, i));
            while let Some(&(m, _)) = minq.back() {
                if m > n {
                    minq.pop_back();
                } else {
                    break;
                }
            }
            minq.push_back((n, i));
            while maxq.front().unwrap().0 - minq.front().unwrap().0 > k {
                let r = xor[s];
                let mut idx = 1;
                for i in (0..15).rev() {
                    let bit = (r >> i) & 1;
                    idx = 2 * idx + bit as usize;
                    trie[idx] -= 1;
                }
                while let Some(&(_, j)) = maxq.front() {
                    if s >= j {
                        maxq.pop_front();
                    } else {
                        break;
                    }
                }
                while let Some(&(_, j)) = minq.front() {
                    if s >= j {
                        minq.pop_front();
                    } else {
                        break;
                    }
                }
                s += 1;
            }
            let mut mx = 0;
            idx = 1;
            let x = xor[i + 1];
            for i in (0..15).rev() {
                let bit = ((x >> i) & 1) as usize;
                if trie[2 * idx + (1 - bit)] > 0 {
                    mx |= 1 << i;
                    idx = 2 * idx + (1 - bit);
                } else {
                    idx = 2 * idx + bit;
                }
            }
            ans = ans.max(mx);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_xor() {
        assert_eq!(Solution::max_xor(vec![3, 5, 4], 2), 6);
        assert_eq!(Solution::max_xor(vec![0, 3], 0), 3);
        assert_eq!(Solution::max_xor(vec![5, 4, 5, 6], 2), 7);
        assert_eq!(Solution::max_xor(vec![5, 4, 5, 6], 1), 6);
    }
}
