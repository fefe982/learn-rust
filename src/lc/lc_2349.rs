// https://leetcode.com/problems/design-a-number-container-system/
// 2349. Design a Number Container System
struct NumberContainers {
    idx: std::collections::HashMap<i32, std::collections::BTreeSet<i32>>,
    num: std::collections::HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            idx: std::collections::HashMap::new(),
            num: std::collections::HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(old) = self.num.get(&index) {
            self.idx.get_mut(old).unwrap().remove(&index);
        }
        self.idx.entry(number).or_default().insert(index);
        self.num.insert(index, number);
    }

    fn find(&self, number: i32) -> i32 {
        self.idx
            .get(&number)
            .map_or(-1, |s| s.iter().next().copied().unwrap_or(-1))
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_containers() {
        let null = 0;
        for test in [(
            [
                "NumberContainers",
                "find",
                "change",
                "change",
                "change",
                "change",
                "find",
                "change",
                "find",
            ],
            vec_vec![[], [10], [2, 10], [1, 10], [3, 10], [5, 10], [10], [1, 20], [10]],
            vec![null, -1, null, null, null, null, 1, null, 2],
        )] {
            let mut obj = NumberContainers::new();
            for ((cmd, args), expect) in test.0.into_iter().zip(test.1).zip(test.2).skip(1) {
                match cmd {
                    "find" => {
                        assert_eq!(obj.find(args[0]), expect);
                    }
                    "change" => {
                        obj.change(args[0], args[1]);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
