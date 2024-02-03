use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Debug)]
pub struct HashTable<K, V> {
    pub container: Vec<HashCell<K, V>>,
    pub cells_in_use_count: usize,
}

pub trait Hash {
    fn hash(&self) -> usize;
}

#[derive(Default, Debug, Clone)]
pub struct HashCell<K, V> {
    pub key: K,
    pub value: V,
    pub in_use: bool,
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
        assert!(self.container.len() > 0);
        // FIXME: remove the * 2 operation it is very slow
        // size of vector should be a prime number
        let mut new_self = Self {
            container: vec![HashCell::<K, V>::default(); self.container.len() * 2],
            cells_in_use_count: 0,
        };
        for elem in self.container.iter() {
            if elem.in_use {
                new_self.insert(elem.key.clone(), elem.value.clone());
            }
        }
        *self = new_self;
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

impl Hash for u32 {
    fn hash(&self) -> usize {
        *self as usize
    }
}
