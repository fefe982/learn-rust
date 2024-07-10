// https://leetcode.com/problems/encrypt-and-decrypt-strings/
// 2227. Encrypt and Decrypt Strings
pub struct Encrypter {
    values: Vec<String>,
    map: std::collections::HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Encrypter {
    fn encrypt_inner(values: &Vec<String>, word: String) -> String {
        let word = word.as_bytes();
        let n = word.len();
        let res = word.iter().fold("".to_string(), |mut acc, c| {
            acc.push_str(values[(c - b'a') as usize].as_str());
            acc
        });
        if res.as_bytes().len() != n * 2 {
            "".to_string()
        } else {
            res
        }
    }
    pub fn new(keys: Vec<char>, values: Vec<String>, dictionary: Vec<String>) -> Self {
        let mut v = vec!["".to_string(); 26];
        for (k, val) in keys.into_iter().zip(values) {
            v[(k as u8 - b'a') as usize] = val;
        }
        let mut m = std::collections::HashMap::<String, i32>::new();
        for word in dictionary {
            let em = Self::encrypt_inner(&v, word);
            if !em.is_empty() {
                *m.entry(em).or_default() += 1;
            }
        }
        Self { values: v, map: m }
    }

    pub fn encrypt(&self, word1: String) -> String {
        Self::encrypt_inner(&self.values, word1)
    }

    pub fn decrypt(&self, word2: String) -> i32 {
        self.map.get(&word2).cloned().unwrap_or(0)
    }
}

/**
 * Your Encrypter object will be instantiated and called as such:
 * let obj = Encrypter::new(keys, values, dictionary);
 * let ret_1: String = obj.encrypt(word1);
 * let ret_2: i32 = obj.decrypt(word2);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_encrypter() {
        let null = ();
        for test in [(
            ["Encrypter", "encrypt", "decrypt"],
            vec_any![
                [
                    ['a', 'b', 'c', 'd'],
                    ["ei", "zf", "ei", "am"],
                    ["abcd", "acbd", "adbc", "badc", "dacb", "cadb", "cbda", "abad"]
                ],
                ["abcd"],
                ["eizfeiam"]
            ],
            vec_any![null, "eizfeiam", 2],
        )] {
            let mut obj = None;
            for ((func, args), ans) in test.0.into_iter().zip(test.1).zip(test.2) {
                match func {
                    "Encrypter" => {
                        obj = Some(Encrypter::new(
                            args[0].as_vec_char(),
                            args[1].as_vec_string(),
                            args[2].as_vec_string(),
                        ));
                    }
                    "encrypt" => {
                        assert_eq!(obj.as_ref().unwrap().encrypt(args[0].as_string()), ans.as_string());
                    }
                    "decrypt" => {
                        assert_eq!(obj.as_ref().unwrap().decrypt(args[0].as_string()), ans.as_i32());
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
