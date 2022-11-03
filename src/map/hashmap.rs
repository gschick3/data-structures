use std::vec;

const BUCKETS:usize = 16;

pub struct HashMap {
    pairs:Vec<Vec<(String, i32)>>
}

impl HashMap {
    pub fn new() -> Self {
        HashMap {
            pairs: vec![vec![]; BUCKETS]
        }
    }

    pub fn from(pairs:Vec<(&str, i32)>) -> Self {
        let mut h = HashMap::new();
        for pair in pairs {
            h.insert(pair.0, pair.1);
        }
        h
    }
    
    pub fn get_keys(&self) -> Vec<&str> {
        let mut keys = vec![];
        for v in &self.pairs {
            for k in v {
                keys.push(k.0.as_str());
            }
        }
        keys
    }

    pub fn get(&self, key:&str) -> Option<i32> {
        let bucket = Self::hash(key);
        for pair in &self.pairs[bucket] {
            if pair.0.as_str() == key {
                return Some(pair.1);
            }
        }
        None
    }

    pub fn insert(&mut self, key:&str, value:i32) {
        let bucket = Self::hash(key);
        match self.pairs[bucket].iter().position(|s| s.0.as_str() == key) {
            Some(i) => self.pairs[bucket][i].1 = value,
            None => self.pairs[bucket].push((String::from(key), value))
        }
    }

    fn hash(key:&str) -> usize {
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
        assert_eq!(HashMap::hash("a"), 97 % 16);
    }

    #[test]
    fn hashmap_from() {
        let h = HashMap::from(vec![("a",1), ("b",2), ("c", 3)]);
        let keys = ["a", "b", "c"];
        assert!(h.get_keys().iter().all(|k| keys.contains(k)));
    }
    
    #[test]
    fn insert_key() {
        let mut h = HashMap::new();
        h.insert("one", 4);
        h.insert("two", 6);
        h.insert("three", 10);

        let keys = ["one", "two", "three"];
        assert!(h.get_keys().iter().all(|k| keys.contains(k)));
    }

    #[test]
    fn get_exists() {
        let mut h = HashMap::new();
        h.insert("abc", 4);
        assert_eq!(h.get("abc"), Some(4));
    }

    #[test]
    fn get_not_exist() {
        let mut h = HashMap::new();
        h.insert("abc", 4);
        assert_eq!(h.get("abb"), None);
    }
}