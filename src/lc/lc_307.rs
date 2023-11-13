// https://leetcode.com/problems/range-sum-query-mutable/
// 307. Range Sum Query - Mutable
pub struct NumArray {
    v: Vec<Vec<i32>>,
    s: Vec<i32>,
    sz: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut sz = (n as f64).sqrt() as usize;
        if sz * sz < n {
            sz += 1;
        }
        let mut v = vec![vec![0; sz]; sz];
        let mut s = vec![0; sz];
        for (i, n) in nums.into_iter().enumerate() {
            v[i / sz][i % sz] = n;
            s[i / sz] += n;
        }
        Self { v, s, sz }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let i = index as usize;
        let diff = val - self.v[i / self.sz][i % self.sz];
        self.v[i / self.sz][i % self.sz] = val;
        self.s[i / self.sz] += diff;
    }

    pub fn sum_range(&mut self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let ileft = left / self.sz;
        let iright = right / self.sz;
        if ileft == iright {
            return self.v[ileft][left % self.sz..=right % self.sz].iter().sum::<i32>();
        }
        let mut sum = self.v[ileft][left % self.sz..].iter().sum::<i32>();
        sum += self.v[iright][0..=right % self.sz].iter().sum::<i32>();
        return sum + self.s[ileft + 1..iright].iter().sum::<i32>();
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_range() {
        let mut obj = NumArray::new(vec![1, 3, 5]);
        assert_eq!(obj.sum_range(0, 2), 9);
        obj.update(1, 2);
        assert_eq!(obj.sum_range(0, 2), 8);
    }
}
