use hash_table::*;

#[test]
fn create_empty_hash_table() {
	let ht: HashTable<String, String> = HashTable::<String, String>::new();
	let ht2: HashTable<u32, String> = HashTable::<u32, String>::new();
	assert_eq!(ht.get(&"".to_string()), None);
	assert_eq!(ht2.get(&0), None);
}

#[test]
fn initialize_hash_table () {
	let mut ht: HashTable<String, String> = HashTable::<String, String>::new();
	let mut ht2: HashTable<u32, String> = HashTable::<u32, String>::new();

	assert_eq!(ht.container.len(), 5);
	assert_eq!(ht2.container.len(), 5);

	for i in 0..100 {
		ht.insert(i.to_string(), format!("{}", 1_000_000 + i));
		ht2.insert(i, format!("{}", 1_000_000 + i));
	}

	assert_eq!(ht.container.len(), 160);
	assert_eq!(ht2.container.len(), 160);

}

#[test]
fn check_for_updated_values () {
	let mut ht: HashTable<String, String> = HashTable::<String, String>::new();
	let mut ht2: HashTable<u32, String> = HashTable::<u32, String>::new();

	for i in 0..100 {
		ht.insert(i.to_string(), format!("{}", 1_000_000 + i));
		ht2.insert(i, format!("{}", 1_000_000 + i));
	}

	assert_eq!(*ht.get(&"0".to_string()).unwrap_or(&"".to_string()), format!("1000000"));
	assert_eq!(*ht2.get(&0).unwrap_or(&"".to_string()), format!("1000000"));

	ht.insert("0".to_string(), "new value".to_string());
	ht2.insert(0, "new value".to_string());

	assert_eq!(*ht.get(&"0".to_string()).unwrap_or(&"".to_string()), format!("new value"));
	assert_eq!(*ht2.get(&0).unwrap_or(&"".to_string()), format!("new value"));

}

