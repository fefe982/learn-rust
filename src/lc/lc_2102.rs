// https://leetcode.com/problems/sequentially-ordinal-rank-tracker/
// 2102. Sequentially Ordinal Rank Tracker
pub struct SORTracker {
    base: std::collections::BinaryHeap<(i32, std::cmp::Reverse<String>)>,
    top: std::collections::BinaryHeap<(std::cmp::Reverse<i32>, String)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SORTracker {
    pub fn new() -> Self {
        Self {
            base: std::collections::BinaryHeap::new(),
            top: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, name: String, score: i32) {
        self.top.push((std::cmp::Reverse(score), name));
        let Some((std::cmp::Reverse(lscore), lname)) = self.top.pop() else {
            unreachable!()
        };
        self.base.push((lscore, std::cmp::Reverse(lname)));
    }

    pub fn get(&mut self) -> String {
        let Some((tscore, std::cmp::Reverse(tname))) = self.base.pop() else {
            unreachable!()
        };
        self.top.push((std::cmp::Reverse(tscore), tname.clone()));
        tname
    }
}

/**
 * Your SORTracker object will be instantiated and called as such:
 * let obj = SORTracker::new();
 * obj.add(name, score);
 * let ret_2: String = obj.get();
 */
#[cfg(test)]
mod tests {
    use super::super::helpers::Any;
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        let null = "";
        for (vop, vparam, vret) in vec![(
            vec![
                "SORTracker",
                "add",
                "add",
                "get",
                "add",
                "get",
                "add",
                "get",
                "add",
                "get",
                "add",
                "get",
                "get",
            ],
            vec_any![
                [],
                ["bradford", 2],
                ["branford", 3],
                [],
                ["alps", 2],
                [],
                ["orland", 2],
                [],
                ["orlando", 3],
                [],
                ["alpine", 2],
                [],
                []
            ],
            vec![
                null, null, null, "branford", null, "alps", null, "bradford", null, "bradford", null, "bradford",
                "orland",
            ],
        )] {
            let mut obj = SORTracker::new();
            for ((op, param), ret) in vop.into_iter().zip(vparam).zip(vret) {
                match op {
                    "SORTracker" => {
                        obj = SORTracker::new();
                    }
                    "add" => {
                        let &[Any::Str(name), Any::I32(score)] = param.as_slice() else {
                            unreachable!()
                        };
                        obj.add(name.to_string(), score);
                    }
                    "get" => {
                        assert_eq!(obj.get(), ret);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
