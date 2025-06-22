// https://leetcode.com/problems/implement-magic-dictionary/
// 676. Implement Magic Dictionary
pub struct MagicDictionary {
    dict: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {
    pub fn new() -> Self {
        Self { dict: vec![] }
    }

    pub fn build_dict(&mut self, dictionary: Vec<String>) {
        self.dict = dictionary
    }

    pub fn search(&self, search_word: String) -> bool {
        let s = search_word.as_bytes();
        'dict: for d in self.dict.iter() {
            if d.len() != s.len() {
                continue;
            }
            let mut diff = false;
            for (c1, c2) in s.iter().zip(d.as_bytes()) {
                if *c1 != *c2 {
                    if diff {
                        continue 'dict;
                    }
                    diff = true;
                }
            }
            if diff {
                return true;
            }
        }
        false
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_magic_dictionay() {
        let null = false;
        for test in [(
            vec!["MagicDictionary", "buildDict", "search", "search", "search", "search"],
            vec_any![
                [],
                [["hello", "leetcode"]],
                ["hello"],
                ["hhllo"],
                ["hell"],
                ["leetcoded"]
            ],
            vec![null, null, false, true, false, false],
        )] {
            let mut obj = MagicDictionary::new();
            for ((method, args), ans) in test.0.into_iter().zip(test.1).zip(test.2) {
                match method {
                    "MagicDictionary" => (),
                    "buildDict" => obj.build_dict(args[0].as_vec_string()),
                    "search" => assert_eq!(obj.search(args[0].as_string()), ans),
                    _ => panic!("Unexpected method: {}", method),
                };
            }
        }
    }
}
