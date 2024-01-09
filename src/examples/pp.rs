use rand::{Rng, thread_rng};
use crate::bt::*;

#[derive(Debug)]
struct BlackBoard {
	thirst: u8,
	bladder: u8,
	drinking: bool,
	pissing: bool,
	occupied: bool
}

impl BlackBoard {
	fn default () -> Self {
		Self { thirst: 100, bladder: 0, drinking: false, pissing: false, occupied: false }
	}
}


fn random() -> u8 { thread_rng().gen_range(0..16) }
fn coin() -> bool { thread_rng().gen_bool(0.5) }

pub fn main () {
	type T = BlackBoard;

	let need2pee: LeafFunc<T> = |data| {
		if data.bladder > 100 { Pass } else { Fail }
	};

	let pee: LeafFunc<T> = |data| {
		let decr = random();
		// TODO: fix mutability
		if data.bladder >= decr { data.bladder -= decr; }
		Wait
	};
}
