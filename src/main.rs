use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Debug)]
struct HashTable<K, V> {
    container: Vec<HashCell<K, V>>,
    cells_in_use_count: usize,
}

trait Hash {
    fn hash(&self) -> usize;
}

#[derive(Default, Debug, Clone)]
struct HashCell<K, V> {
    key: K,
    value: V,
    in_use: bool,
}

impl Hash for String {
    // dhb2 hash function
    // http://www.cse.yorku.ca/~oz/hash.html
    fn hash(&self) -> usize {
        let mut hash: usize = 5381;
        for c in self.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as usize);
        }
        hash
    }
}

impl<K: Default + Clone + Hash + Debug + PartialEq, V: Default + Clone + Debug> HashTable<K, V> {
    pub fn new() -> Self {
        // capacity is a prime number to reduce collisions
        const CONTAINER_CAPACITY: usize = 5;
        Self {
            container: vec![HashCell::<K, V>::default(); CONTAINER_CAPACITY],
            cells_in_use_count: 0,
        }
    }

    fn extend(&mut self) {
        
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.cells_in_use_count == self.container.len() {
            self.extend();
        }
        assert!(self.cells_in_use_count < self.container.len());

        if let Some(old_value) = self.get_mut(&key) {
            Some(std::mem::replace(old_value, value))
        } else {
            assert!(self.cells_in_use_count < self.container.len());

            let mut index = key.hash() % self.container.len();

            while self.container[index].in_use {
                index = (index + 1) % self.container.len();
            }

            self.container[index].in_use = true;
            self.container[index].key = key;
            self.container[index].value = value;
            self.cells_in_use_count += 1;
            None
        }
    }

    fn get_index(&self, key: &K) -> Option<usize> {
        let mut index = key.hash() % self.container.len();
        for _i in 0..self.container.len() {
            if !self.container[index].in_use {
                break;
            }
            if self.container[index].key == *key {
                break;
            }
            index = (index + 1) % self.container.len();
        }
        if self.container[index].in_use && self.container[index].key == *key {
            Some(index)
        } else {
            None
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_index(key)
            .map(|index| &self.container[index].value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_index(key)
            .map(|index| &mut self.container[index].value)
    }

    //print information about the container
    fn debug(&self) {
        for cell in self.container.iter() {
            if cell.in_use {
                println!("{:#?} -> {:#?}", cell.key, cell.value);
            } else {
                println!("empty cell");
            }
        }
    }
}

fn main() {
    let mut ht: HashTable<String, String> = HashTable::<String, String>::new();
    ht.insert("Hello".to_string(), "world".to_string());
    ht.insert("test".to_string(), "1234".to_string());
    let res = ht.insert("Hello".to_string(), "12345".to_string());

    println!("get : {:?}", ht.get(&"Hello".to_string()));
    println!("get : {:?}", ht.get(&"test".to_string()));
    ht.debug();
    println!("{:#?}", res);
}
