use crate::bt::*;

#[derive(Debug)]
struct BlackBoard {
	thirst: u8,
	bladder: u8,
	drinking: bool,
	pissing: bool
}

impl BlackBoard {
	fn default () -> Self {
		Self { thirst: 100, bladder: 0, drinking: false, pissing: false }
	}
}

pub fn main () {
	let tree = tree!(BlackBoard, Sequence(vec![]));
}
