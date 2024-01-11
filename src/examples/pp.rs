use std::default;

use rand::{Rng, thread_rng};
use crate::bt::*;


#[derive(Debug, Default)]
enum PersonState {
	#[default]
	Drinking,
	NeedToPee, Waiting, Pissing, Thirsty
}

#[derive(Debug, Default)]
struct BlackBoard {
	state: PersonState,
	thirst: u8,
	bladder: u8,
	occupied: bool
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
		if data.bladder >= decr { data.bladder -= decr; }
		Wait
	};
}
