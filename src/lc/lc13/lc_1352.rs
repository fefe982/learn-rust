// https://leetcode.com/problems/product-of-the-last-k-numbers/
// 1352. Product of the Last K Numbers
pub struct ProductOfNumbers {
    prod: Vec<i32>,
    zero: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    pub fn new() -> Self {
        Self {
            prod: vec![1],
            zero: vec![0],
        }
    }

    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.prod.push(1);
            self.zero.push(self.zero.last().unwrap() + 1);
        } else {
            self.prod.push(self.prod.last().unwrap() * num);
            self.zero.push(*self.zero.last().unwrap());
        }
    }

    pub fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if self.zero[self.zero.len() - k - 1] < self.zero[self.zero.len() - 1] {
            0
        } else {
            self.prod[self.prod.len() - 1] / self.prod[self.prod.len() - 1 - k]
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_of_numbers() {
        let mut obj = ProductOfNumbers::new();
        obj.add(3);
        obj.add(0);
        obj.add(2);
        obj.add(5);
        obj.add(4);
        assert_eq!(obj.get_product(2), 20);
        assert_eq!(obj.get_product(3), 40);
        assert_eq!(obj.get_product(4), 0);
        obj.add(8);
        assert_eq!(obj.get_product(2), 32);
    }
}
