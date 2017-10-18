//! Bidirectional maps for Rust.
//!
//! # Examples
//!
//! ```
//! use bidir_map::{BidirMap, ByFirst, BySecond};
//! use std::default::Default;
//!
//! let mut map = BidirMap::new();
//! assert_eq!(map, Default::default());
//!
//! map.insert(1, "a");
//!
//! assert_eq!(map.get_by_first(&1), Some(&"a"));
//! assert_eq!(map.get_by_first(&2), None);
//! assert_eq!(map.get_by_second(&"a"), Some(&1));
//! assert_eq!(map.get_by_second(&"b"), None);
//!
//! assert_eq!(map[ByFirst(&1)], "a");
//! assert_eq!(map[BySecond(&"a")], 1);
//! // These would panic:
//! //   map[ByFirst(&2)];
//! //   map[BySecond(&"b")];
//! ```


use std::borrow::Borrow;
use std::slice;
use std::iter::{Extend, FromIterator};
use std::ops::Index;
use std::vec;


/// Create a `BidirMap` from a set of K/V-K/V pairs.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate bidir_map;
/// # fn main() {
/// let map = bidir_map!(
///     "a" => 1,
///     "b" => 2,
/// );
///
/// assert_eq!(map.get_by_first(&"a"), Some(&1));
/// assert_eq!(map.get_by_second(&2),  Some(&"b"));
/// assert_eq!(map.get_by_first(&"c"), None);
/// # let mut best_map = bidir_map::BidirMap::new();
/// # best_map.insert("a", 1);
/// # best_map.insert("b", 2);
/// # assert_eq!(map, best_map);
/// # }
/// ```
#[macro_export]
macro_rules! bidir_map {
	(@single $($x:tt)*) => (());
	(@count $($rest:expr),*) => (<[()]>::len(&[$(bidir_map!(@single $rest)),*]));

	// Ideally the separator would be <=> instead of => but it's parsed as <= > and therefore illegal
	($($key:expr => $value:expr,)+) => { bidir_map!($($key => $value),+) };
	($($key:expr => $value:expr),*) => {{
		let cap = bidir_map!(@count $($key),*);
		let mut map = ::bidir_map::BidirMap::with_capacity(cap);
		$(map.insert($key, $value);)*
		map
	}};
}


/// A bidirectional map.
///
/// Bidirectional maps allow for mapping from and to both types.
///
/// The interface is based on that of `BTreeMap`, except, that for all functions, where one would supply a key, there are two functions,
/// each treating one of the types as keys (`get()` -> `get_by_{first,second}()`).
///
/// Performance: `O(n)`, mostly.
#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct BidirMap<Kv1: PartialEq, Kv2: PartialEq> {
	cont: Vec<(Kv1, Kv2)>,
}

impl<Kv1: PartialEq, Kv2: PartialEq> BidirMap<Kv1, Kv2> {
	/// Create a new empty instance of `BidirMap`
	pub fn new() -> Self {
		BidirMap{
			cont: Vec::new(),
		}
	}

	/// Create a new empty instance of `BidirMap` with the specified capacity.
	///
	/// It will be able to hold at least `capacity` elements without reallocating.
	pub fn with_capacity(capacity: usize) -> Self {
		BidirMap{
			cont: Vec::with_capacity(capacity),
		}
	}

	/// Clears the map, removing all entries.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// a.insert(1, "a");
	/// a.clear();
	/// assert!(a.is_empty());
	/// ```
	pub fn clear(&mut self) {
		self.cont.clear()
	}

	/// Inserts a K/V-K/V pair into the map.
	///
	/// If the map did not have this K/V-K/V pair present, `None` is returned.
	///
	/// If the map did have this K/V-K/V pair present, it's updated and the old K/V-K/V pair is returned.
	pub fn insert(&mut self, kv1: Kv1, kv2: Kv2) -> Option<(Kv1, Kv2)> {
		let retval =
			if self.contains_first_key(&kv1) {
				self.remove_by_first(&kv1)
			} else if self.contains_second_key(&kv2) {
				self.remove_by_second(&kv2)
			} else {
				None
			};

		self.cont.push((kv1, kv2));

		retval
	}

	/// Gets an iterator over the entries of the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// map.insert(2, "b");
	/// map.insert(3, "c");
	///
	/// for kv in map.iter() {
	/// 	println!("{}: {}", kv.0, kv.1);
	/// }
	///
	/// let first = map.iter().next().unwrap();
	/// assert_eq!(first, (&1, &"a"));
	/// ```
	pub fn iter(&self) -> Iter<Kv1, Kv2> {
		Iter{
			iter: self.cont.iter(),
		}
	}

	/// Gets a mutable iterator over the entries of the map.
	///
	/// # Examples
	///
	/// ```
	/// # #[macro_use]
	/// # extern crate bidir_map;
	/// # fn main() {
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert("a", 1);
	/// map.insert("b", 2);
	/// map.insert("c", 3);
	///
	/// // add 10 to the value if the key isn't "a"
	/// for kv in map.iter_mut() {
	/// 	if *kv.0 != "a" {
	/// 		*kv.1 += 10;
	/// 	}
	/// }
	/// # assert_eq!(map, bidir_map!["a" => 1, "b" => 12, "c" => 13]);
	/// # }
	/// ```
	pub fn iter_mut(&mut self) -> IterMut<Kv1, Kv2> {
		IterMut{
			iter: self.cont.iter_mut(),
		}
	}

	/// Gets an iterator over the first K/V of the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// a.insert(1, "a");
	/// a.insert(2, "b");
	///
	/// let keys: Vec<_> = a.first_col().cloned().collect();
	/// assert_eq!(keys, [1, 2]);
	/// ```
	pub fn first_col(&self) -> FirstColumn<Kv1, Kv2> {
		FirstColumn{
			iter: self.cont.iter(),
		}
	}

	/// Gets an iterator over the second K/V of the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// a.insert(1, "a");
	/// a.insert(2, "b");
	///
	/// let keys: Vec<_> = a.second_col().cloned().collect();
	/// assert_eq!(keys, ["a", "b"]);
	/// ```
	pub fn second_col(&self) -> SecondColumn<Kv1, Kv2> {
		SecondColumn{
			iter: self.cont.iter(),
		}
	}

	/// Returns the number of elements in the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// assert_eq!(a.len(), 0);
	/// a.insert(1, "a");
	/// assert_eq!(a.len(), 1);
	/// ```
	pub fn len(&self) -> usize {
		self.cont.len()
	}

	/// Returns true if the map contains no elements.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut a = BidirMap::new();
	/// assert!(a.is_empty());
	/// a.insert(1, "a");
	/// assert!(!a.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.cont.is_empty()
	}


	/// Returns a reference to the second K/V corresponding to the first K/V.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get_by_first(&1), Some(&"a"));
	/// assert_eq!(map.get_by_first(&2), None);
	/// ```
	pub fn get_by_first<Q: ?Sized>(&self, key: &Q) -> Option<&Kv2>
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter().find(|&kvs| *key == kvs.0).map(|ref kvs| &kvs.1)
	}

	/// Returns a reference to the first K/V corresponding to the second K/V.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.get_by_second(&"a"), Some(&1));
	/// assert_eq!(map.get_by_second(&"b"), None);
	/// ```
	pub fn get_by_second<Q: ?Sized>(&self, key: &Q) -> Option<&Kv1>
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter().find(|&kvs| *key == kvs.1).map(|ref kvs| &kvs.0)
	}

	/// Check if the map contains the first K/V
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.contains_first_key(&1), true);
	/// assert_eq!(map.contains_first_key(&2), false);
	/// ```
	pub fn contains_first_key<Q: ?Sized>(&self, key: &Q) -> bool
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter().any(|ref kvs| *key == kvs.0)
	}

	/// Check if the map contains the second K/V
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.contains_second_key(&"a"), true);
	/// assert_eq!(map.contains_second_key(&"b"), false);
	/// ```
	pub fn contains_second_key<Q: ?Sized>(&self, key: &Q) -> bool
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter().any(|ref kvs| *key == kvs.1)
	}

	/// Returns a mutable reference to the second K/V corresponding to the first K/V.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// if let Some(x) = map.get_mut_by_first(&1) {
	///     *x = "b";
	/// }
	/// assert_eq!(map.get_by_first(&1), Some(&"b"));
	/// ```
	pub fn get_mut_by_first<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Kv2>
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter_mut().find(|ref kvs| *key == kvs.0).map(|&mut (_, ref mut kv2)| kv2)
	}

	/// Returns a mutable reference to the first K/V corresponding to the second K/V.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// if let Some(x) = map.get_mut_by_second(&"a") {
	///     *x = 2;
	/// }
	/// assert_eq!(map.get_by_second(&"a"), Some(&2));
	/// ```
	pub fn get_mut_by_second<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut Kv1>
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter_mut().find(|ref kvs| *key == kvs.1).map(|&mut (ref mut kv1, _)| kv1)
	}

	/// Removes the pair corresponding to the first K/V from the map, returning it if the key was previously in the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.remove_by_first(&1), Some((1, "a")));
	/// assert_eq!(map.remove_by_first(&1), None);
	/// ```
	pub fn remove_by_first<Q: ?Sized>(&mut self, key: &Q) -> Option<(Kv1, Kv2)>
		where Kv1: Borrow<Q>,
		      Q  : PartialEq<Kv1>,
	{
		self.cont.iter().position(|ref kvs| *key == kvs.0).map(|idx| self.cont.swap_remove(idx))
	}

	/// Removes the pair corresponding to the first K/V from the map, returning it if the key was previously in the map.
	///
	/// # Examples
	///
	/// ```
	/// use bidir_map::BidirMap;
	///
	/// let mut map = BidirMap::new();
	/// map.insert(1, "a");
	/// assert_eq!(map.remove_by_second(&"a"), Some((1, "a")));
	/// assert_eq!(map.remove_by_second(&"b"), None);
	/// ```
	pub fn remove_by_second<Q: ?Sized>(&mut self, key: &Q) -> Option<(Kv1, Kv2)>
		where Kv2: Borrow<Q>,
		      Q  : PartialEq<Kv2>,
	{
		self.cont.iter().position(|ref kvs| *key == kvs.1).map(|idx| self.cont.swap_remove(idx))
	}
}


impl<Kv1: PartialEq, Kv2: PartialEq> IntoIterator for BidirMap<Kv1, Kv2> {
	type Item = (Kv1, Kv2);
	type IntoIter = vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		return self.cont.into_iter()
	}
}

impl<Kv1: PartialEq, Kv2: PartialEq> FromIterator<(Kv1, Kv2)> for BidirMap<Kv1, Kv2> {
	fn from_iter<T: IntoIterator<Item=(Kv1, Kv2)>>(iter: T) -> Self {
		BidirMap{
			cont: Vec::from_iter(iter),
		}
	}
}

impl<Kv1: PartialEq, Kv2: PartialEq> Extend<(Kv1, Kv2)> for BidirMap<Kv1, Kv2> {
	fn extend<T: IntoIterator<Item=(Kv1, Kv2)>>(&mut self, iter: T) {
		self.cont.extend(iter)
	}
}


/// Wrapper type for getting second keys/values with first keys/values via `Index`.
///
/// To assume that by-first is the "default" would be incorrect, so thus you one can wrap their indices with this and all is swell.
///
/// # Examples
///
/// ```
/// use bidir_map::{BidirMap, ByFirst};
///
/// let mut map = BidirMap::new();
/// map.insert(1, "a");
/// assert_eq!(map[ByFirst(&1)], "a");
/// assert_eq!(map[&ByFirst(&1)], "a");
/// ```
pub struct ByFirst<'q, Q: ?Sized + 'q>(pub &'q Q);

/// Wrapper type for getting second keys/values with first keys/values via `Index`.
///
/// # Examples
///
/// ```
/// use bidir_map::{BidirMap, BySecond};
///
/// let mut map = BidirMap::new();
/// map.insert(1, "a");
/// assert_eq!(map[BySecond(&"a")], 1);
/// assert_eq!(map[&BySecond(&"a")], 1);
/// ```
pub struct BySecond<'q, Q: ?Sized + 'q>(pub &'q Q);

impl<'q, Kv1: PartialEq, Kv2: PartialEq, Q: ?Sized + 'q> Index<ByFirst<'q, Q>> for BidirMap<Kv1, Kv2>
	where Kv1: Borrow<Q>,
	      Q  : PartialEq<Kv1>,
{
	type Output = Kv2;
	fn index(&self, key: ByFirst<Q>) -> &Self::Output {
		self.get_by_first(&key.0).expect("no entry found for first key/value")
	}
}

impl<'a, 'q, Kv1: PartialEq, Kv2: PartialEq, Q: ?Sized + 'q> Index<&'a ByFirst<'q, Q>> for BidirMap<Kv1, Kv2>
	where Kv1: Borrow<Q>,
	      Q  : PartialEq<Kv1>,
{
	type Output = Kv2;
	fn index(&self, key: &ByFirst<Q>) -> &Self::Output {
		self.get_by_first(&key.0).expect("no entry found for first key/value")
	}
}

impl<'q, Kv1: PartialEq, Kv2: PartialEq, Q: ?Sized + 'q> Index<BySecond<'q, Q>> for BidirMap<Kv1, Kv2>
	where Kv2: Borrow<Q>,
	      Q  : PartialEq<Kv2>,
{
	type Output = Kv1;
	fn index(&self, key: BySecond<Q>) -> &Self::Output {
		self.get_by_second(&key.0).expect("no entry found for second key/value")
	}
}

impl<'a, 'q, Kv1: PartialEq, Kv2: PartialEq, Q: ?Sized + 'q> Index<&'a BySecond<'q, Q>> for BidirMap<Kv1, Kv2>
	where Kv2: Borrow<Q>,
	      Q  : PartialEq<Kv2>,
{
	type Output = Kv1;
	fn index(&self, key: &BySecond<Q>) -> &Self::Output {
		self.get_by_second(&key.0).expect("no entry found for second key/value")
	}
}


/// An iterator over the K/V pairs contained in a `BidirMap`.
///
/// See documentation of [`BidirMap::iter()`](struct.BidirMap.html#method.iter) for more.
pub struct Iter<'a, Kv1: 'a, Kv2: 'a> {
	iter: slice::Iter<'a, (Kv1, Kv2)>,
}

impl<'a, Kv1, Kv2> Iterator for Iter<'a, Kv1, Kv2> {
	type Item = (&'a Kv1, &'a Kv2);
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|&(ref kv1, ref kv2)| (kv1, kv2))
	}
}


/// An iterator over mutable K/V pairs contained in a `BidirMap`.
///
/// See documentation of [`BidirMap::iter_mut()`](struct.BidirMap.html#method.iter_mut) for more.
pub struct IterMut<'a, Kv1: 'a, Kv2: 'a> {
	iter: slice::IterMut<'a, (Kv1, Kv2)>,
}

impl<'a, Kv1, Kv2> Iterator for IterMut<'a, Kv1, Kv2> {
	type Item = (&'a mut Kv1, &'a mut Kv2);
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|&mut (ref mut kv1, ref mut kv2)| (kv1, kv2))
	}
}


/// An iterator the first set of K/Vs in a `BidirMap`.
///
/// See documentation of [`BidirMap::first_col()`](struct.BidirMap.html#method.first_col) for more.
pub struct FirstColumn<'a, Kv1: 'a, Kv2: 'a> {
	iter: slice::Iter<'a, (Kv1, Kv2)>,
}

impl<'a, Kv1, Kv2> Iterator for FirstColumn<'a, Kv1, Kv2> {
	type Item = &'a Kv1;
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|ref kvs| &kvs.0)
	}
}


/// An iterator the second set of K/Vs in a `BidirMap`.
///
/// See documentation of [`BidirMap::second_col()`](struct.BidirMap.html#method.second_col) for more.
pub struct SecondColumn<'a, Kv1: 'a, Kv2: 'a> {
	iter: slice::Iter<'a, (Kv1, Kv2)>,
}

impl<'a, Kv1, Kv2> Iterator for SecondColumn<'a, Kv1, Kv2> {
	type Item = &'a Kv2;
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|ref kvs| &kvs.1)
	}
}
