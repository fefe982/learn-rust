// https://leetcode.com/problems/design-an-ordered-stream/
// 1656. Design an Ordered Stream
pub struct OrderedStream {
    v: Vec<Option<String>>,
    p: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    pub fn new(n: i32) -> Self {
        Self {
            v: vec![None; n as usize],
            p: 0,
        }
    }

    pub fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.v[(id_key - 1) as usize] = Some(value);
        if self.v[self.p].is_some() {
            let r: Vec<String> = self.v[self.p..].iter_mut().map_while(|s| s.take()).collect();
            self.p += r.len();
            r
        } else {
            vec![]
        }
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ordered_stream() {
        let mut os = OrderedStream::new(5);
        assert_eq!(os.insert(3, "ccccc".to_string()), vec![] as Vec<String>);
        assert_eq!(os.insert(1, "aaaaa".to_string()), vec!["aaaaa"]);
        assert_eq!(os.insert(2, "bbbbb".to_string()), vec!["bbbbb", "ccccc"]);
        assert_eq!(os.insert(5, "eeeee".to_string()), vec![] as Vec<String>);
        assert_eq!(os.insert(4, "ddddd".to_string()), vec!["ddddd", "eeeee"]);
    }
}
