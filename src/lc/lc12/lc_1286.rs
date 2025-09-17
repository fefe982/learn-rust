// https://leetcode.com/problems/iterator-for-combination/
// 1286. Iterator for Combination
struct CombinationIterator {
    characters: Vec<char>,
    index: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combination_length: i32) -> Self {
        Self {
            characters: characters.chars().collect(),
            index: (0..combination_length as usize).collect(),
        }
    }

    fn next(&mut self) -> String {
        let mut ret = String::new();
        for i in &self.index {
            ret.push(self.characters[*i]);
        }
        let mut i = self.index.len();
        while i > 0 && self.index[i - 1] + self.index.len() - i + 1 == self.characters.len() {
            i -= 1;
        }
        if i > 0 {
            self.index[i - 1] += 1;
            for j in i..self.index.len() {
                self.index[j] = self.index[j - 1] + 1;
            }
        } else {
            self.index.truncate(0);
        }
        ret
    }

    fn has_next(&self) -> bool {
        self.index.len() != 0
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combination_iterator() {
        let mut obj = CombinationIterator::new("abc".to_string(), 2);
        assert_eq!(obj.next(), "ab".to_string());
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), "ac".to_string());
        assert_eq!(obj.has_next(), true);
        assert_eq!(obj.next(), "bc".to_string());
        assert_eq!(obj.has_next(), false);
    }
}
