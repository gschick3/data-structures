use std::vec;

const BUCKETS:usize = 16;

pub struct HashMap {
    pairs:Vec<Vec<(String, i32)>>
}

impl HashMap {
    pub fn new() -> HashMap {
        HashMap {
            pairs: vec![vec![]; BUCKETS]
        }
    }
    
    pub fn get_keys(&self) -> Vec<String> {
        let mut keys = vec![];
        for v in &self.pairs {
            for k in v {
                keys.push(String::from(&k.0));
            }
        }
        keys
    }

    pub fn get(&self, key:String) -> Option<i32> {
        let bucket = Self::hash(&String::from(&key));
        for pair in &self.pairs[bucket] {
            if pair.0 == key {
                return Some(pair.1);
            }
        }
        None
    }

    pub fn insert(&mut self, key:String, value:i32) {
        let bucket = Self::hash(&String::from(&key));
        match self.pairs[bucket].iter().position(|s| s.0 == key) {
            Some(i) => self.pairs[bucket][i].1 = value,
            None => self.pairs[bucket].push((key, value))
        }
    }

    fn hash(key:&String) -> usize {
        let mut n:u32 = 0;
        key.chars().for_each(|c| n += c as u32);
        n as usize % 16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn get_keys_empty() {
        let h = HashMap::new();
        assert_eq!(h.get_keys(), Vec::<String>::new());
    }

    #[test]
    fn hash_char() {
        assert_eq!(HashMap::hash(&String::from("a")), 97 % 16);
    }

    #[test]
    fn get_exists() {
        let mut h = HashMap::new();
        h.insert(String::from("abc"), 4);
        assert_eq!(h.get(String::from("abc")).unwrap(), 4);
    }

    #[test]
    fn get_not_exist() {
        let mut h = HashMap::new();
        h.insert(String::from("abc"), 4);
        assert_eq!(h.get(String::from("abb")), None);
    }

    #[test]
    fn insert_key() {
        let mut h = HashMap::new();
        h.insert(String::from("abc"), 4);
        h.insert(String::from("abc"), 6);
        h.insert(String::from("abb"), 10);

        for key in h.get_keys() {
            assert_ne!(h.get(key), None);
        }

        assert_eq!(h.get_keys(), vec![String::from("abb"), String::from("abc")]);
    }
}