// https://leetcode.com/problems/all-oone-data-structure/
// 432. All O`one Data Structure
struct Cnt {
    cnt: i32,
    prev: usize,
    next: usize,
    words: std::collections::HashSet<String>,
}
pub struct AllOne {
    cnts: Vec<Cnt>,
    str_map: std::collections::HashMap<String, i32>,
    cnt_map: std::collections::HashMap<i32, usize>,
    free_list: Vec<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    pub fn new() -> Self {
        let mut w = std::collections::HashSet::new();
        w.insert(String::from(""));
        Self {
            cnts: vec![
                Cnt {
                    cnt: 0,
                    prev: usize::MAX,
                    next: 1,
                    words: w.clone(),
                },
                Cnt {
                    cnt: i32::MAX,
                    prev: 0,
                    next: usize::MAX,
                    words: w.clone(),
                },
            ],
            str_map: std::collections::HashMap::new(),
            cnt_map: std::collections::HashMap::new(),
            free_list: vec![],
        }
    }

    fn remove(&mut self, key: &String, cnt: i32) -> (usize, usize, usize) {
        let &cnt_idx = self.cnt_map.get(&cnt).unwrap();
        self.cnts[cnt_idx].words.remove(key);
        let Cnt { prev, next, .. } = self.cnts[cnt_idx];
        if self.cnts[cnt_idx].words.is_empty() {
            self.cnt_map.remove(&cnt);
            self.cnts[prev].next = next;
            self.cnts[next].prev = prev;
            self.free_list.push(cnt_idx);
            (prev, usize::MAX, next)
        } else {
            (prev, cnt_idx, next)
        }
    }

    fn insert(&mut self, key: &String, cnt: i32, mut prev: usize, cur: usize, mut next: usize) {
        let cnt_idx = self.cnt_map.entry(cnt).or_default();
        if *cnt_idx == 0 {
            if cur != usize::MAX {
                if self.cnts[cur].cnt > cnt {
                    next = cur;
                } else {
                    prev = cur;
                }
            }
            if let Some(idx) = self.free_list.pop() {
                *cnt_idx = idx;
                self.cnts[idx].cnt = cnt;
                self.cnts[idx].prev = prev;
                self.cnts[idx].next = next;
            } else {
                *cnt_idx = self.cnts.len();
                self.cnts.push(Cnt {
                    cnt,
                    prev,
                    next,
                    words: std::collections::HashSet::new(),
                });
            }
            self.cnts[prev].next = *cnt_idx;
            self.cnts[next].prev = *cnt_idx;
        }
        self.cnts[*cnt_idx].words.insert(key.clone());
    }

    pub fn inc(&mut self, key: String) {
        let ocnt = self.str_map.get(&key);
        if let Some(&cnt) = ocnt {
            let (prev, cur, next) = self.remove(&key, cnt);
            self.insert(&key, cnt + 1, prev, cur, next);
            self.str_map.insert(key, cnt + 1);
        } else {
            self.insert(&key, 1, 0, self.cnts[0].next, usize::MAX);
            self.str_map.insert(key, 1);
        }
    }

    pub fn dec(&mut self, key: String) {
        let &cnt = self.str_map.get(&key).unwrap();
        let (prev, cur, next) = self.remove(&key, cnt);
        if cnt == 1 {
            self.str_map.remove(&key);
        } else {
            self.insert(&key, cnt - 1, prev, cur, next);
            self.str_map.insert(key, cnt - 1);
        }
    }

    pub fn get_max_key(&self) -> String {
        self.cnts[self.cnts[1].prev]
            .words
            .iter()
            .next()
            .unwrap()
            .clone()
    }

    pub fn get_min_key(&self) -> String {
        self.cnts[self.cnts[0].next]
            .words
            .iter()
            .next()
            .unwrap()
            .clone()
    }
}
/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allone() {
        let mut obj = AllOne::new();
        obj.inc(String::from("a"));
        obj.inc(String::from("b"));
        obj.inc(String::from("b"));
        obj.inc(String::from("b"));
        obj.inc(String::from("b"));
        obj.dec(String::from("b"));
        obj.dec(String::from("b"));
        assert_eq!(obj.get_max_key(), String::from("b"));
        assert_eq!(obj.get_min_key(), String::from("a"));
        let mut obj = AllOne::new();
        obj.inc(String::from("hello"));
        obj.inc(String::from("hello"));
        assert_eq!(obj.get_max_key(), String::from("hello"));
        assert_eq!(obj.get_min_key(), String::from("hello"));
        obj.inc(String::from("leet"));
        assert_eq!(obj.get_max_key(), String::from("hello"));
        assert_eq!(obj.get_min_key(), String::from("leet"));
    }
}
