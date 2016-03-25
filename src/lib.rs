use std::borrow::Borrow;
use std::slice;
use std::iter::{Extend, FromIterator};
use std::vec;


#[derive(Clone, Debug, Hash, PartialEq, Eq)]
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

	pub fn clear(&mut self) {
		self.cont.clear()
	}

	pub fn insert(&mut self, kv1: Kv1, kv2: Kv2) {
		self.cont.push((kv1, kv2));
	}

	pub fn iter<'a>(&'a self) -> slice::Iter<'a, (Kv1, Kv2)> {
		self.cont.iter()
	}

	pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, (Kv1, Kv2)> {
		self.cont.iter_mut()
	}

	//TODO: maybe implement keys() and values() as first_row() and second_row()?

	pub fn len(&self) -> usize {
		self.cont.len()
	}

	pub fn is_empty(&self) -> bool {
		self.cont.is_empty()
	}
}


impl<Kv1: PartialEq, Kv2: PartialEq> BidirMap<Kv1, Kv2> {
	pub fn get<'q, Q>(&self, key: &'q Q) -> Option<&Kv2>
		where Kv1  : Borrow<Q>,
		      &'q Q: PartialEq<Kv1>,
	{
		self.cont.iter().find(|&kvs| key == kvs.0).map(|ref kvs| &kvs.1)
	}

	pub fn contains_key<'q, Q>(&self, key: &'q Q) -> bool
		where Kv1  : Borrow<Q>,
		      &'q Q: PartialEq<Kv1>,
	{
		self.cont.iter().any(|ref kvs| key == kvs.0)
	}

	//TODO: implement get_mut
	/*pub fn get_mut<'q, Q>(&mut self, key: &'q Q) -> Option<&mut Kv2>
		where Kv1  : Borrow<Q>,
		      &'q Q: PartialEq<Kv1>,
	{
		self.cont.iter_mut().find(|&mut kvs| key == kvs.0).map(|&mut kvs| &mut kvs.1)
	}*/

	pub fn remove<'q, Q>(&mut self, key: &'q Q) -> Option<Kv1>
		where Kv2  : Borrow<Q>,
		      &'q Q: PartialEq<Kv2>,
	{
		self.cont.iter().position(|ref kvs| key == kvs.1).map(|idx| self.cont.swap_remove(idx).0)
	}
}

impl<Kv1: PartialEq, Kv2: PartialEq> BidirMap<Kv1, Kv2> {
	pub fn get<'q, Q>(&self, key: &'q Q) -> Option<&Kv1>
		where Kv2  : Borrow<Q>,
		      &'q Q: PartialEq<Kv2>,
	{
		self.cont.iter().find(|&kvs| key == kvs.1).map(|ref kvs| &kvs.0)
	}

	pub fn contains_key<'q, Q>(&self, key: &'q Q) -> bool
		where Kv2  : Borrow<Q>,
		      &'q Q: PartialEq<Kv2>,
	{
		self.cont.iter().any(|ref kvs| key == kvs.1)
	}

	//TODO: implement get_mut
	/*pub fn get_mut<'q, Q>(&mut self, key: &'q Q) -> Option<&mut Kv1>
		where Kv2  : Borrow<Q>,
		      &'q Q: PartialEq<Kv2>,
	{
		self.cont.iter_mut().find(|&mut kvs| key == kvs.1).map(|&mut kvs| &mut kvs.0)
	}*/

	pub fn remove<'q, Q>(&mut self, key: &'q Q) -> Option<Kv2>
		where Kv1  : Borrow<Q>,
		      &'q Q: PartialEq<Kv1>,
	{
		self.cont.iter().position(|ref kvs| key == kvs.0).map(|idx| self.cont.swap_remove(idx).1)
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
