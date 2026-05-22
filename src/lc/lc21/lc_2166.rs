// https://leetcode.com/problems/design-bitset/
// 2166. Design Bitset
pub struct Bitset {
    bits: Vec<bool>,
    flipped: bool,
    ones: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bitset {
    pub fn new(size: i32) -> Self {
        Self {
            bits: vec![false; size as usize],
            flipped: false,
            ones: 0,
        }
    }

    pub fn fix(&mut self, idx: i32) {
        let idx = idx as usize;
        if self.bits[idx] == self.flipped {
            self.bits[idx] = !self.flipped;
            self.ones += 1;
        }
    }

    pub fn unfix(&mut self, idx: i32) {
        let idx = idx as usize;
        if self.bits[idx] != self.flipped {
            self.bits[idx] = self.flipped;
            self.ones -= 1;
        }
    }

    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
        self.ones = self.bits.len() as i32 - self.ones;
    }

    pub fn all(&self) -> bool {
        self.ones == self.bits.len() as i32
    }

    pub fn one(&self) -> bool {
        self.ones > 0
    }

    pub fn count(&self) -> i32 {
        self.ones
    }

    pub fn to_string(&self) -> String {
        self.bits
            .iter()
            .map(|&bit| if bit ^ self.flipped { '1' } else { '0' })
            .collect()
    }
}

/**
 * Your Bitset object will be instantiated and called as such:
 * let obj = Bitset::new(size);
 * obj.fix(idx);
 * obj.unfix(idx);
 * obj.flip();
 * let ret_4: bool = obj.all();
 * let ret_5: bool = obj.one();
 * let ret_6: i32 = obj.count();
 * let ret_7: String = obj.to_string();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bitset() {
        let mut obj = Bitset::new(5);
        obj.fix(3);
        obj.fix(1);
        assert_eq!(obj.to_string(), "01010");
        obj.flip();
        assert_eq!(obj.to_string(), "10101");
        assert_eq!(obj.all(), false);
        assert_eq!(obj.one(), true);
        assert_eq!(obj.count(), 3);
        obj.unfix(0);
        assert_eq!(obj.to_string(), "00101");
        obj.flip();
        assert_eq!(obj.to_string(), "11010");
        assert_eq!(obj.one(), true);
        obj.unfix(0);
        assert_eq!(obj.count(), 2);
        assert_eq!(obj.to_string(), "01010");
    }

    #[test]
    fn test_bitset_idempotent_operations() {
        let mut obj = Bitset::new(1);
        assert_eq!(obj.all(), false);
        assert_eq!(obj.one(), false);
        assert_eq!(obj.count(), 0);
        obj.fix(0);
        obj.fix(0);
        assert_eq!(obj.all(), true);
        assert_eq!(obj.count(), 1);
        obj.flip();
        assert_eq!(obj.to_string(), "0");
        obj.unfix(0);
        assert_eq!(obj.count(), 0);
    }
}
