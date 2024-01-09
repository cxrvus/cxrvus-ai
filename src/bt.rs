use std::ops::Deref;

use rand::{Rng, thread_rng};


#[derive(Debug, PartialEq)]
enum State { Fail, Pass, Wait, Error(String) }

#[derive(Debug)]
enum Node {
	Sequence (Vec<Node>),
	Fallback (Vec<Node>),
	Passer (Vec<Node>),
	Random (Vec<Node>),

	Inverter (Box<Node>),

	Flow (Vec<Node>, FlowFunc),
	Leaf (LeafFunc),
}



#[derive(Debug)]
struct Tree<T> { root: Node, data: T }


type FlowFunc = fn(&Vec<Node>) -> State;
type LeafFunc = fn() -> State;

use State::*;


impl<T> Tree<T> {
	pub fn tick(&self) -> State {
		let mut data: T = self.data;
		let mut stack: Vec<(Node, usize)> = vec![];
		let mut state: State;


		let push = |&node| { stack.push((node, 0)) };
		let pop = || { stack.pop().expect("prevented by while") };
		let peek = || { stack.last().expect("prevented by while") };
		let cont = || {
			let (x,i) = pop();
			stack.push((x,i+1))
		};


		let sequence: FlowFunc = |nodes| {
			continue_on(Pass, nodes)
		};

		let fallback: FlowFunc = |nodes| {
			continue_on(Fail, nodes)
		};

		// let continue_on = |nodes: &Vec<Node>, cont_state: State| {
		// 	for node in nodes {
		// 		let child_state = node.tick();
		// 		if child_state == cont_state { continue; }
		// 		else { return child_state;}
		// 	}
		// 	return cont_state
		// };


		let passer: FlowFunc = |nodes| {
			for node in nodes {
				if let Error(error) = node.tick() { return Error(error); }
			};
			return Pass;
		};

		let inverter = |node: &Node| {
			match node.tick() {
				Pass => Fail,
				Fail => Pass,
				other => other
			}	
		};


		let random: FlowFunc = |nodes| {
			let mut rng = thread_rng();
			let index = rng.gen_range(0..nodes.len());
			nodes[index].tick()
		};

		while let Some((node, i)) = stack.last() {
			state = match node {
				Node::Sequence(nodes) => sequence(&nodes),
				Node::Fallback(nodes) => fallback(&nodes),
				Node::Passer(nodes) => passer(&nodes),
				Node::Random(nodes) => random(&nodes),
				Node::Inverter(node) => inverter(node.deref()),
				_ => Error("unhandled node type".into())
			}
		}

		Error("".into())
	}
}



struct BB { thirst_level: u8, pee_level: u8 }

fn main () {
}
