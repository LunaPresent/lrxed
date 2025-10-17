use core::cmp::Ordering;

use derive_more::Deref;

use super::{KeyChord, KeySequence};

type MapIdx = u16;

/// Denotes a mapping from [`KeySequence`] to a generic [`AppEvent`][ae]
///
/// [ae]: crate::tui::event::AppEvent
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyMapping<T> {
	/// The complete key sequence
	pub key_sequence: KeySequence,
	/// The event that should be triggered
	pub app_event: T,
}

impl<T> KeyMapping<T> {
	/// Creates a new `KeyMapping`
	pub fn new(key_sequence: KeySequence, app_event: T) -> Self {
		Self {
			key_sequence,
			app_event,
		}
	}
}

/// A key-value map with keys of type [`KeySequence`]
/// and values representing a generic [`AppEvent`][ae]
///
/// # Examples
/// ```
/// # use oprabeli::config::*;
/// // Example key map; the strings in the second argument represent an app event
/// let key_mappings = vec![
///     KeyMapping::new("G".parse().unwrap(), "cursor to bottom"),
///     KeyMapping::new("gd".parse().unwrap(), "goto definition"),
///     KeyMapping::new("gg".parse().unwrap(), "cursor to top"),
///     KeyMapping::new("go".parse().unwrap(), "cursor to top"),
///     KeyMapping::new("k".parse().unwrap(), "cursor up"),
///     KeyMapping::new("j".parse().unwrap(), "cursor down"),
///     KeyMapping::new("l".parse().unwrap(), "cursor right"),
///     KeyMapping::new("h".parse().unwrap(), "cursor left"),
///     KeyMapping::new("hi".parse().unwrap(), "omg hiiii"),
/// ];
/// let key_map = KeyMap::from(key_mappings.clone());
///
/// // Filtering a single length key sequence mapping
/// let filtered = key_map.match_key(KeyChord::from_char('G'), KeyMapMatch::new());
/// assert_eq!(filtered.matches(&key_map), &key_mappings[0..1]); // [G]
/// assert_eq!(filtered.full_matches(&key_map), &key_mappings[0..1]); // [G]
///
/// // Filtering a multi length key sequence mapping
/// let mut filtered = key_map.match_key(KeyChord::from_char('g'), KeyMapMatch::new());
/// assert_eq!(filtered.matches(&key_map), &key_mappings[1..4]); // [gd, gg, go]
/// assert_eq!(filtered.full_matches(&key_map), &[]);
/// filtered = key_map.match_key(KeyChord::from_char('g'), filtered);
/// assert_eq!(filtered.matches(&key_map), &key_mappings[2..3]); // [gg]
/// assert_eq!(filtered.full_matches(&key_map), &key_mappings[2..3]); // [gg]
///
/// let mut filtered = key_map.match_key(KeyChord::from_char('h'), KeyMapMatch::new());
/// assert_eq!(filtered.matches(&key_map), &key_mappings[7..9]); // [h, hi]
/// assert_eq!(filtered.full_matches(&key_map), &key_mappings[7..8]); // [h]
/// filtered = key_map.match_key(KeyChord::from_char('i'), filtered);
/// assert_eq!(filtered.matches(&key_map), &key_mappings[8..9]); // [hi]
/// assert_eq!(filtered.full_matches(&key_map), &key_mappings[8..9]); // [hi]
/// ```
///
/// [ae]: crate::tui::event::AppEvent
#[derive(Debug, Clone, PartialEq, Eq, Deref)]
pub struct KeyMap<T>(Vec<KeyMapping<T>>);

impl<T> From<Vec<KeyMapping<T>>> for KeyMap<T> {
	fn from(value: Vec<KeyMapping<T>>) -> Self {
		Self(value).sorted()
	}
}

impl<T> KeyMap<T> {
	/// Filters the key map by a next key chord in a sequence
	///
	/// The first key in a sequence can be matched by passing in a blank [`KeyMapMatch::new`]
	/// into *prev_match*. The filtered range will be recorded into the returned [`KeyMapMatch`].
	/// Further keys can then be filtered by passing that `KeyMapMatch` back into *prev_match*.
	///
	/// The `KeyMapMatch` does not actually retain information on pressed keys, nor does it
	/// remember which `KeyMap` created it. Reusing the same `KeyMapMatch` across different
	/// `KeyMap`s will produce undefined behaviour, though it will never panic.
	///
	/// See [`KeyMapMatch::partial_matches`] and [`KeyMapMatch::full_matches`]
	/// for reading the actual range of filtered mappings.
	pub fn match_key(&self, key_chord: KeyChord, prev_match: KeyMapMatch) -> KeyMapMatch {
		let mut match_start = prev_match.match_start as usize;
		let mut match_end = (prev_match.match_end as usize).min(self.len());
		let next_key_idx = prev_match.next_key_idx as usize;

		let match_idx = loop {
			let middle = (match_end + match_start) / 2;
			if middle >= self.len() {
				break None;
			}
			match self[middle]
				.key_sequence
				.get(next_key_idx)
				.cmp(&Some(&key_chord))
			{
				Ordering::Equal => break Some(middle),
				Ordering::Greater => {
					match_end = middle;
				}
				Ordering::Less => {
					match_start = middle + 1;
				}
			}
			if match_start >= match_end {
				break None;
			}
		};

		let mut full_match_end = 0;
		if let Some(idx) = match_idx {
			// unless intentionally stress testing, the amount of matching mappings should be
			// reasonably small here, therefore let's linear search starting from `match_idx`
			for idx in (match_start..idx).rev() {
				if self[idx].key_sequence.get(next_key_idx) != Some(&key_chord) {
					match_start = idx + 1;
					break;
				}
			}
			for idx in (idx + 1)..match_end {
				if self[idx].key_sequence.get(next_key_idx) != Some(&key_chord) {
					match_end = idx;
					break;
				}
			}

			// as None is less than Some(_), shorter sequences always come at the front
			// therefore the full match slice will always be at the front
			full_match_end = match_end;
			for idx in match_start..match_end {
				if self[idx].key_sequence.len() > (next_key_idx + 1) {
					full_match_end = idx;
					break;
				}
			}
		}

		KeyMapMatch {
			match_start: match_start as MapIdx,
			match_end: match_end as MapIdx,
			full_match_end: full_match_end as MapIdx,
			next_key_idx: prev_match.next_key_idx + 1,
		}
	}

	fn sorted(mut self) -> Self {
		self.0
			.sort_unstable_by(|lhs, rhs| lhs.key_sequence.cmp(&rhs.key_sequence));
		self
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyMapMatch {
	match_start: MapIdx,
	match_end: MapIdx,
	full_match_end: MapIdx,
	next_key_idx: u8,
}

impl Default for KeyMapMatch {
	fn default() -> Self {
		Self {
			match_start: 0,
			match_end: MapIdx::MAX,
			full_match_end: 0,
			next_key_idx: 0,
		}
	}
}

impl KeyMapMatch {
	/// Creates a new "blank" match
	///
	/// This is equivalent to [`KeyMapMatch::default`]
	pub fn new() -> Self {
		Self::default()
	}

	/// Returns all partially or fully matching key mappings
	///
	/// A match is any key mapping for which the first n key chords of the key
	/// sequence are equal to all n key chords recorded in this `KeyMapMatch`, in order
	pub fn matches<T>(self, key_map: &KeyMap<T>) -> &[KeyMapping<T>] {
		let from = self.match_start as usize;
		let to = (self.match_end as usize).min(key_map.len());
		&key_map.0[from..to]
	}

	/// Returns all fully matching key mappings
	///
	/// This is a subset of the set returned by [`KeyMapMatch::matches`],
	/// To be precise: any match where the key sequence has the same length as
	/// the amount of key chords recorded in this `KeyMapMatch` is a full match
	pub fn full_matches<T>(self, key_map: &KeyMap<T>) -> &[KeyMapping<T>] {
		let from = self.match_start as usize;
		let to = self.full_match_end as usize;
		&key_map.0[from..to]
	}

	/// Returns all fully matching key mappings
	///
	/// This is a subset of the set returned by [`KeyMapMatch::matches`],
	/// Any match that isn't a full match is a partial match
	pub fn partial_matches<T>(self, key_map: &KeyMap<T>) -> &[KeyMapping<T>] {
		let from = self.full_match_end as usize;
		let to = (self.match_end as usize).min(key_map.len());
		&key_map.0[from..to]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_key_map() -> KeyMap<i32> {
		KeyMap::from(
			[
				"ba", "zz", "f", "b", "bb", "goo", "a", "y", "gz", "zb", "go", "goa", "za",
			]
			.iter()
			.map(|s| KeyMapping {
				key_sequence: s.parse().unwrap(),
				app_event: 0,
			})
			.collect::<Vec<_>>(),
		)
	}

	#[test]
	fn sort_on_create() {
		let key_map = create_key_map();
		let sorted = [
			"a", "b", "ba", "bb", "f", "go", "goa", "goo", "gz", "y", "za", "zb", "zz",
		];

		for (key_mapping, reference) in key_map.0.iter().zip(sorted.into_iter()) {
			assert_eq!(key_mapping.key_sequence.to_string(), reference);
		}
	}

	#[test]
	fn match_key_random() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('b'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 1);
		assert_eq!(key_match.match_end, 4);
		assert_eq!(key_match.full_match_end, 2);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('a'), key_match);

		assert_eq!(key_match.match_start, 2);
		assert_eq!(key_match.match_end, 3);
		assert_eq!(key_match.full_match_end, 3);
		assert_eq!(key_match.next_key_idx, 2);

		let key_match = key_map.match_key(KeyChord::from_char('z'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 3);
	}

	#[test]
	fn match_key_middle() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('g'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 5);
		assert_eq!(key_match.match_end, 9);
		assert_eq!(key_match.full_match_end, 5);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('o'), key_match);

		assert_eq!(key_match.match_start, 5);
		assert_eq!(key_match.match_end, 8);
		assert_eq!(key_match.full_match_end, 6);
		assert_eq!(key_match.next_key_idx, 2);

		let key_match = key_map.match_key(KeyChord::from_char('o'), key_match);

		assert_eq!(key_match.match_start, 7);
		assert_eq!(key_match.match_end, 8);
		assert_eq!(key_match.full_match_end, 8);
		assert_eq!(key_match.next_key_idx, 3);

		let key_match = key_map.match_key(KeyChord::from_char('s'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 4);
	}

	#[test]
	fn match_key_first() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('a'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 0);
		assert_eq!(key_match.match_end, 1);
		assert_eq!(key_match.full_match_end, 1);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('a'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 2);
	}

	#[test]
	fn match_key_last() {
		let key_map = create_key_map();
		let key_match = key_map.match_key(KeyChord::from_char('z'), KeyMapMatch::new());

		assert_eq!(key_match.match_start, 10);
		assert_eq!(key_match.match_end, 13);
		assert_eq!(key_match.full_match_end, 10);
		assert_eq!(key_match.next_key_idx, 1);

		let key_match = key_map.match_key(KeyChord::from_char('z'), key_match);

		assert_eq!(key_match.match_start, 12);
		assert_eq!(key_match.match_end, 13);
		assert_eq!(key_match.full_match_end, 13);
		assert_eq!(key_match.next_key_idx, 2);

		let key_match = key_map.match_key(KeyChord::from_char('z'), key_match);

		assert!(key_match.match_start >= key_match.match_end);
		assert_eq!(key_match.next_key_idx, 3);
	}

	#[test]
	fn matches() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch {
			match_start: 5,
			match_end: 8,
			full_match_end: 6,
			next_key_idx: 2,
		};

		let matches = key_match.matches(&key_map);
		assert_eq!(matches.len(), 3);
		assert_eq!(matches[0].key_sequence.to_string(), "go");
		assert_eq!(matches[1].key_sequence.to_string(), "goa");
		assert_eq!(matches[2].key_sequence.to_string(), "goo");
	}

	#[test]
	fn matches_blank() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch::new();

		let matches = key_match.matches(&key_map);
		assert_eq!(matches.len(), 13);
	}

	#[test]
	fn full_matches() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch {
			match_start: 5,
			match_end: 8,
			full_match_end: 6,
			next_key_idx: 2,
		};

		let matches = key_match.full_matches(&key_map);
		assert_eq!(matches.len(), 1);
		assert_eq!(matches[0].key_sequence.to_string(), "go");
	}

	#[test]
	fn full_matches_blank() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch::new();

		let matches = key_match.full_matches(&key_map);
		assert_eq!(matches.len(), 0);
	}

	#[test]
	fn partial_matches() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch {
			match_start: 5,
			match_end: 8,
			full_match_end: 6,
			next_key_idx: 2,
		};

		let matches = key_match.partial_matches(&key_map);
		assert_eq!(matches.len(), 2);
		assert_eq!(matches[0].key_sequence.to_string(), "goa");
		assert_eq!(matches[1].key_sequence.to_string(), "goo");
	}

	#[test]
	fn partial_matches_blank() {
		let key_map = create_key_map();
		let key_match = KeyMapMatch::new();

		let matches = key_match.partial_matches(&key_map);
		assert_eq!(matches.len(), 13);
	}
}
