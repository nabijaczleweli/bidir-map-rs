extern crate bidir_map;

use bidir_map::BidirMap;

/// https://github.com/nabijaczleweli/bidir-map-rs/issues/1
///
/// Compilation test
#[test]
fn get_with_borrowed_from_unsized_works() {
	let mut map: BidirMap<String, usize> = BidirMap::new();
	map.insert("asdf".to_string(), 1234);
	assert_eq!(map.get_by_first("asdf"), Some(&1234));
}
