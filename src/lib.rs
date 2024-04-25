#![cfg_attr(not(test), no_std)]

/// digests `inp` into `sbox` in-place.
/// single-pass (1 round).
///
/// `op(&sbox[i], inp[i])`.
/// `op` can be any `fn` that acts like a binary operator.
/// in the future, it may be required to be commutative,
/// allowing argument/operand order to be implementation-defined.
pub fn hasher_single<'a, T, I, F>(inp: I, sbox: &mut [T], op: F)
where
	T: 'a,
	I: IntoIterator<Item = &'a T>,
	F: Fn(&T, &'a T) -> T,
{
	if sbox.is_empty() {
		return;
	};
	let mut i: usize = 0;
	for b in inp {
		sbox[i] = op(&sbox[i], b);

		// rustc should easily optimize this
		//i = (i + 1) % sbox.len()
		i += 1;
		// is this branch worse than `%`?
		if i >= sbox.len() {
			i = 0;
		};
	}
}

//#[derive(Clone)] // is this a good idea?
pub struct Hasher<'a, T> {
	/// `next` index to digest into
	i: usize,
	/// stateful box that contains current digest
	sbox: &'a mut [T],
}
impl<'a, T> Hasher<'a, T> {
	/// `iv`: initialization vector.
	/// `start`: initial vector index to begin digesting into.
	pub fn new(start: usize, iv: &'a mut [T]) -> Self {
		Hasher { i: start, sbox: iv }
	}
}
/*
impl<'a> Default for Hasher<'a, u8> {
	fn default() -> Self {
		let mut iv = [0u8; 8];
		Self::new(0, &mut iv)
	}
}
*/
impl<'a, T> Iterator for Hasher<'a, T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(4, 4);
	}
}
