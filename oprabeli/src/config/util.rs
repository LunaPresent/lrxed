use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum OneOrMany<T> {
	One(T),
	Many(Vec<T>),
}

impl<T> OneOrMany<T> {
	pub fn len(&self) -> usize {
		match self {
			Self::One(_) => 1,
			Self::Many(items) => items.len(),
		}
	}
}

impl<'a, T> IntoIterator for &'a OneOrMany<T> {
	type Item = &'a T;

	type IntoIter = OneOrManyIterator<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		OneOrManyIterator {
			container: self,
			idx: 0,
		}
	}
}

#[derive(Debug)]
pub(crate) struct OneOrManyIterator<'a, T> {
	container: &'a OneOrMany<T>,
	idx: usize,
}

impl<'a, T> Iterator for OneOrManyIterator<'a, T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		let item = match self.container {
			OneOrMany::One(item) if self.idx == 0 => Some(item),
			OneOrMany::Many(items) => items.get(self.idx),
			_ => None,
		};

		self.idx += 1;

		item
	}
}
