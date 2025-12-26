// https://leetcode.com/problems/maximum-strong-pair-xor-ii/
// 2935. Maximum Strong Pair XOR II
pub struct Solution;
struct Trie {
    children: [Option<Box<Trie>>; 2],
}
impl Trie {
    fn new() -> Self {
        Self { children: [None, None] }
    }
    fn insert(&mut self, num: i32) {
        self.insert_mask(num, 1 << 19);
    }
    fn insert_mask(&mut self, num: i32, mask: i32) {
        if mask == 0 {
            return;
        }
        let bit = ((num & mask) != 0) as usize;
        if self.children[bit].is_none() {
            self.children[bit] = Some(Box::new(Trie::new()));
        }
        self.children[bit].as_mut().unwrap().insert_mask(num, mask >> 1);
    }
    fn remove(&mut self, num: i32) {
        self.remove_mask(num, 1 << 19);
    }
    fn remove_mask(&mut self, num: i32, mask: i32) -> bool {
        if mask == 0 {
            return true;
        }
        let bit = ((num & mask) != 0) as usize;
        if self.children[bit].is_some() {
            if self.children[bit].as_mut().unwrap().remove_mask(num, mask >> 1) {
                self.children[bit] = None;
            }
        }
        self.children[0].is_none() && self.children[1].is_none()
    }
    fn query(&self, num: i32) -> i32 {
        self.query_mask(num, 1 << 19)
    }
    fn query_mask(&self, num: i32, mask: i32) -> i32 {
        if mask == 0 {
            return 0;
        }
        let bit = ((num & mask) != 0) as usize;
        if self.children[bit ^ 1].is_some() {
            return self.children[bit ^ 1].as_ref().unwrap().query_mask(num, mask >> 1) | mask;
        } else {
            return self.children[bit].as_ref().unwrap().query_mask(num, mask >> 1);
        }
    }
}
impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut trie = Trie::new();
        let mut j = 0;
        let mut ans = 0;
        for i in 0..nums.len() {
            trie.insert(nums[i]);
            while nums[j] * 2 < nums[i] {
                trie.remove(nums[j]);
                j += 1;
            }
            ans = ans.max(trie.query(nums[i]));
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_strong_pair_xor() {
        assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![10, 100]), 0);
        assert_eq!(Solution::maximum_strong_pair_xor(vec![500, 520, 2500, 3000]), 1020);
    }
}
