use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Cow;

struct CaseInsensitiveMap<V> {
    map: HashMap<String, V>,
}

impl<V> CaseInsensitiveMap<V> {
    fn new() -> Self {
        CaseInsensitiveMap {
            map: HashMap::new(),
        }
    }

    fn insert<S>(&mut self, key: S, value: V)
        where
            S: Into<String>,
    {
        let key = key.into().to_lowercase();
        self.map.insert(key, value);
    }

    fn get<'a, S>(&'a self, key: S) -> Option<&'a V>
        where
            S: AsRef<str>,
    {
        let key = key.as_ref().to_lowercase();
        self.map.get(&key)
    }

    fn contains_key<S>(&self, key: S) -> bool
        where
            S: AsRef<str>,
    {
        let key = key.as_ref().to_lowercase();
        self.map.contains_key(&key)
    }

    // Additional methods can be added as needed
}

// Example usage:
fn main() {
    let mut my_map = CaseInsensitiveMap::new();

    my_map.insert("Key1", 42);
    my_map.insert(String::from("Key2"), 77);

    println!("Value for key1: {:?}", my_map.get("Key1"));
    println!("Value for key2: {:?}", my_map.get("Key2"));
    println!("Contains key 'key1': {}", my_map.contains_key("key1"));
    println!("Contains key 'key3': {}", my_map.contains_key("key3"));
}
