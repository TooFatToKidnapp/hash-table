#[derive(Debug)]
struct HashTable<K, V> {
    container: Vec<(K, V)>
}

#[derive(Clone, Debug)]
struct  Type {
    i : usize
}

impl  Default for Type {
    fn default() -> Self {
        Self {
            i: 50
        }
    }
}

trait Default {
    fn default() -> Self;
}

impl Default for String {
    fn default() -> Self {
        Self::from("custom default value")
    }
}

impl<K: Default + Clone, V: Default + Clone> HashTable<K, V> {
    fn new() -> Self {
        const CONTAINER_CAPACITY: usize = 64;
        Self {
            container : vec![(K::default(), V::default()); CONTAINER_CAPACITY]
        }
    }
    fn insert(key: K, value: V) {
        todo!();
    }
    fn get(key: &K) -> &V {
        todo!();
    }
    fn get_mut(key: &K) -> &mut V {
        todo!();
    }
}

fn main() {
    let mut ht = HashTable::<Type, String>::new();
    println!("{:#?}", ht);
}
