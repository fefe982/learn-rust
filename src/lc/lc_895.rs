// https://leetcode.com/problems/maximum-frequency-stack/
// 895. Maximum Frequency Stack
struct FreqStack {
    freq: std::collections::HashMap<i32, i32>,
    cnt: std::collections::HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            freq: std::collections::HashMap::new(),
            cnt: std::collections::HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, val: i32) {
        let freq = self.freq.entry(val).or_default();
        *freq += 1;
        self.max_freq = self.max_freq.max(*freq);
        self.cnt.entry(*freq).or_default().push(val);
    }

    fn pop(&mut self) -> i32 {
        let freq = self.max_freq;
        let val = self.cnt.get_mut(&freq).unwrap().pop().unwrap();
        if self.cnt.get(&freq).unwrap().is_empty() {
            self.max_freq -= 1;
        }
        *self.freq.get_mut(&val).unwrap() -= 1;
        val
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut obj = FreqStack::new();
        obj.push(5);
        obj.push(7);
        obj.push(5);
        obj.push(7);
        obj.push(4);
        obj.push(5);
        assert_eq!(obj.pop(), 5);
        assert_eq!(obj.pop(), 7);
        assert_eq!(obj.pop(), 5);
        assert_eq!(obj.pop(), 4);
    }
}
