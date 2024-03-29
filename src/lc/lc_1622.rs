// https://leetcode.com/problems/fancy-sequence/
// 1622. Fancy Sequence
const M: i64 = 1000000007;
pub struct Fancy {
    v: Vec<i64>,
    mul: i64,
    add: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    pub fn new() -> Self {
        Self {
            v: vec![],
            mul: 1,
            add: 0,
        }
    }

    pub fn append(&mut self, val: i32) {
        let mut val = (val as i64 + M - self.add) % M;
        let mut d = self.mul;
        while d != 1 {
            let m = M / d + 1;
            val = (val * m) % M;
            d = (d * m) % M;
        }
        self.v.push(val);
    }

    pub fn add_all(&mut self, inc: i32) {
        self.add = (self.add + inc as i64) % M;
    }

    pub fn mult_all(&mut self, m: i32) {
        self.mul = (self.mul * m as i64) % M;
        self.add = (self.add * m as i64) % M;
    }

    pub fn get_index(&self, idx: i32) -> i32 {
        let idx = idx as usize;
        if idx < self.v.len() {
            (((self.v[idx] * self.mul) % M + self.add) % M) as i32
        } else {
            -1
        }
    }
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fancy_list() {
        let mut fancy = Fancy::new();
        fancy.append(2); // fancy sequence: [2]
        fancy.add_all(3); // fancy sequence: [2+3] -> [5]
        fancy.append(7); // fancy sequence: [5, 7]
        fancy.mult_all(2); // fancy sequence: [5*2, 7*2] -> [10, 14]
        assert_eq!(fancy.get_index(0), 10); // return 10
        fancy.add_all(3); // fancy sequence: [10+3, 14+3] -> [13, 17]
        fancy.append(10); // fancy sequence: [13, 17, 10]
        fancy.mult_all(2); // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]
        assert_eq!(fancy.get_index(0), 26); // return 26
        assert_eq!(fancy.get_index(1), 34); // return 34
        assert_eq!(fancy.get_index(2), 20); // return 20
    }
}
