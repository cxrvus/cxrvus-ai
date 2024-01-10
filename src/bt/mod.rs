use rand::{Rng, thread_rng};

pub use Node::*;
pub use State::*;


#[derive(Debug, PartialEq)]
pub enum State { Fail, Pass, Wait, Error(Option<String>) }

#[derive(Debug)]
pub enum Node<T> {
	Leaf 		(LeafFunc<T>),
	Check 		(CheckFunc<T>),
	Flow 		(Vec<Node<T>>, FlowFunc<T>),
	Sequence 	(Vec<Node<T>>),
	Fallback 	(Vec<Node<T>>),
	Random 		(Vec<Node<T>>),
	Root 		(Box<Node<T>>),
	Inverter 	(Box<Node<T>>),
	Passer 		(Box<Node<T>>),
}


pub type FlowFunc<T> = fn(&Vec<Node<T>>, data: &mut T) -> State;
pub type DecFunc<T> = fn(&Node<T>, data: &mut T) -> State;
pub type LeafFunc<T> = fn(data: &mut T) -> State;
pub type CheckFunc<T> = fn(data: &mut T) -> bool;


impl<T> Node<T> {
	pub fn tick(&self, data: &mut T) -> State {

		let sequence: FlowFunc<T> = |nodes, data| {
			for node in nodes {
				match node.tick(data) {
					Pass => continue,
					other => return other
				}
			}
			Pass
		};

		let fallback: FlowFunc<T> = |nodes, data| {
			for node in nodes {
				match node.tick(data) {
					Fail => continue,
					other => return other
				}
			}
			Fail
		};

		let random: FlowFunc<T> = |nodes, data| {
			let mut rng = thread_rng();
			let index = rng.gen_range(0..nodes.len());
			nodes[index].tick(data)
		};


		let root: DecFunc<T> = |node, data| {
			loop {
				match node.tick(data) {
					error @ Error(_) => return error,
					_ => {}
				}
			}
		};

		let inverter: DecFunc<T> = |node, data| {
			match node.tick(data) {
				Pass => Fail,
				Fail => Pass,
				other => other
			}	
		};

		let passer: DecFunc<T> = |node, data| {
			match node.tick(data) {
				error @ Error(_) => error,
				_ => Pass
			}
		};


		match self {
			Node::Leaf(func) => func(data),
			Node::Check(func) => if func(data) { Pass } else { Fail },
			Node::Flow(nodes, func) => func(nodes, data),
			Node::Sequence(nodes) => sequence(nodes, data),
			Node::Fallback(nodes) => fallback(nodes, data),
			Node::Random(nodes) => random(nodes, data),
			Node::Root(nodes) => root(nodes, data),
			Node::Inverter(node) => inverter(node, data),
			Node::Passer(node) => passer(node, data),
		}
	}
}


#[macro_export]

macro_rules! tree {
	($t: ty, $x: expr) => { Root::<$t>(Box::new($x)) };
}
pub use tree;
