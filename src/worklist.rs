use std::collections::VecDeque;

pub trait Worklist<T: PartialEq>: Default {
	fn empty(&self) -> bool;

	fn insert(&mut self, val: T) -> Option<T>;

	fn extract(&mut self) -> Option<T>;

	fn contains(&self, val: T) -> bool;
}

/// Underlying data storage provided by vec::VecDeque.
#[derive(Debug)]
pub struct FiloWorklist<T> {
	data: VecDeque<T>,
}

#[derive(Debug)]
pub struct FifoWorklist<T> {
	data: VecDeque<T>,
}

/// First-in Last-out data management.
impl<T: Clone + PartialEq> Worklist<T> for FiloWorklist<T> {
	fn empty(&self) -> bool { self.data.is_empty() }

	/// No duplicates.
	fn insert(&mut self, val: T) -> Option<T> {
		if !self.data.contains(&val) {
			self.data.push_front(val.clone());

			Some(val)
		} else {
			None
		}
	}

	fn extract(&mut self) -> Option<T> { self.data.pop_back() }

	fn contains(&self, val: T) -> bool { self.data.contains(&val)}
}

impl<T> Default for FiloWorklist<T> {
	fn default() -> Self {
		FiloWorklist {data: VecDeque::<T>::new()}
	}
}

/// First-in First-out data management.
impl<T: Clone + PartialEq> Worklist<T> for FifoWorklist<T> {
	fn empty(&self) -> bool { self.data.is_empty() }

	/// No duplicates.
	fn insert(&mut self, val: T) -> Option<T> {
		if !self.data.contains(&val) {
			self.data.push_front(val.clone());

			Some(val)
		} else {
			None
		}
	}

	fn extract(&mut self) -> Option<T> { self.data.pop_front() }

	fn contains(&self, val: T) -> bool { self.data.contains(&val)}
}

impl<T> Default for FifoWorklist<T> {
	fn default() -> Self {
		FifoWorklist {data: VecDeque::<T>::new()}
	}
}