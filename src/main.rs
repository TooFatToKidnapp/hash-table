use std::time::Instant;
use rand::random;

fn custom_hash_map_benchmark(elemnt_count: usize) {
    use hash_table::HashTable;
    let mut ht = HashTable::<u32, usize>::new();
    for _ in 0..elemnt_count {
        let key = random::<u32>();
        if let Some(value) = ht.get_mut(&key) {
            *value += 1;
        } else {
            ht.insert(key, 1);
        }
    }}

fn std_hash_map_benchmark(elemnt_count: usize) {
    use std::collections::HashMap;
    let mut ht = HashMap::<u32, usize>::new();
    for _ in 0..elemnt_count {
        let key = random::<u32>();
        if let Some(value) = ht.get_mut(&key) {
            *value += 1;
        } else {
            ht.insert(key, 1);
        }
    }
}

fn main() {
    const ELEMENT_COUNT: usize = 100_000;

    let std_time = Instant::now();
    std_hash_map_benchmark(ELEMENT_COUNT);
    println!("std HashMap Table: {}", std_time.elapsed().as_secs_f32());

    let custom_ht_time = Instant::now();
    custom_hash_map_benchmark(ELEMENT_COUNT);
    println!("custom HashTable: {}", custom_ht_time.elapsed().as_secs_f32());
}

