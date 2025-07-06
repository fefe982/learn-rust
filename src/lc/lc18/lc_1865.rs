// https://leetcode.com/problems/finding-pairs-with-a-certain-sum/
// 1865. Finding Pairs With a Certain Sum
pub struct FindSumPairs {
    map1: std::collections::HashMap<i32, i32>,
    nums2: Vec<i32>,
    map2: std::collections::HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut map1 = std::collections::HashMap::new();
        for i in nums1 {
            *map1.entry(i).or_insert(0) += 1;
        }
        let mut map2 = std::collections::HashMap::new();
        for &i in nums2.iter() {
            *map2.entry(i).or_insert(0) += 1;
        }
        Self { map1, nums2, map2 }
    }

    pub fn add(&mut self, index: i32, val: i32) {
        let index = index as usize;
        *self.map2.entry(self.nums2[index]).or_insert(0) -= 1;
        self.nums2[index] += val;
        *self.map2.entry(self.nums2[index]).or_insert(0) += 1;
    }

    pub fn count(&self, tot: i32) -> i32 {
        self.map1
            .iter()
            .map(|(&k, &v)| self.map2.get(&(tot - k)).unwrap_or(&0) * v)
            .sum::<i32>()
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1865() {
        let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(obj.count(7), 8);
        obj.add(3, 2);
        assert_eq!(obj.count(8), 2);
        assert_eq!(obj.count(4), 1);
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(obj.count(7), 11);
    }
}
