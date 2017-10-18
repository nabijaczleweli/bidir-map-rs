extern crate bidir_map;

use bidir_map::{BidirMap, ByFirst, BySecond};

/// https://github.com/nabijaczleweli/bidir-map-rs/issues/1
///
/// Compilation test
#[test]
fn get_with_borrowed_from_unsized_works() {
	let mut map: BidirMap<String, usize> = BidirMap::new();
	map.insert("asdf".to_string(), 1234);
	assert_eq!(map.get_by_first("asdf"), Some(&1234));
}

#[test]
#[should_panic(expected="no entry found for first key/value")]
fn nonexistant_index_by_first_panics() {
	let mut map: BidirMap<String, usize> = BidirMap::new();
	map.insert("asdf".to_string(), 1234);
	map[ByFirst("fdsa")];
}

#[test]
#[should_panic(expected="no entry found for first key/value")]
fn nonexistant_ref_index_by_first_panics() {
	let mut map: BidirMap<String, usize> = BidirMap::new();
	map.insert("asdf".to_string(), 1234);
	map[&ByFirst("fdsa")];
}

#[test]
#[should_panic(expected="no entry found for second key/value")]
fn nonexistant_index_by_second_panics() {
	let mut map: BidirMap<String, usize> = BidirMap::new();
	map.insert("asdf".to_string(), 1234);
	let _ = map[BySecond(&4321)];
}

#[test]
#[should_panic(expected="no entry found for second key/value")]
fn nonexistant_ref_index_by_second_panics() {
	let mut map: BidirMap<String, usize> = BidirMap::new();
	map.insert("asdf".to_string(), 1234);
	let _ = map[&BySecond(&4321)];
}
