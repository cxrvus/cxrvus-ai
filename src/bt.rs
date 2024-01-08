use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum State { Fail, Pass, Wait, Error(String) }

#[derive(Debug)]
enum Node<T> { Branch(Vec<Node<T>>, fn(&Vec<Node<T>>, &T) -> State), Leaf(fn(&T) -> State) }

enum Value { String(String), Number(i32), Bool(bool) }

type BlackBoard = HashMap<String, Value>;

impl<T> Node<T> {
    fn tick(&self, board: &T) -> State {
        match self {
            Node::Branch(children, func) => func(children, board),
            Node::Leaf(func) => func(board),
        }
    }

	fn root_tick(&self, board: &T) -> String {
		loop {
			if let State::Error(error) = self.tick(board) {  }
			else { continue; };
		}
	}

    fn sequence(children: Vec<Node<T>>) -> Self { Node::Branch(children, branch_funcs::sequence) }

    fn fallback(children: Vec<Node<T>>) -> Self { Node::Branch(children, branch_funcs::fallback) }

    fn passer(children: Vec<Node<T>>) -> Self { Node::Branch(children, branch_funcs::passer) }

    fn inverter(child: Node<T>) -> Self { Node::Branch(vec![child], branch_funcs::inverter) }
}

mod branch_funcs {
	use super::*;
	use State::*;


	pub fn sequence<T>(children: &Vec<Node<T>>, board: &T) -> State {
		continue_on(Pass, children, board)
	}

	pub fn fallback<T>(children: &Vec<Node<T>>, board: &T) -> State {
		continue_on(Fail, children, board)
	}

	fn continue_on<T>(cont_state: State, children: &Vec<Node<T>>, board: &T) -> State {
		for child in children {
			let child_state = child.tick(board);
			if child_state == cont_state { continue; }
			else { return child_state;}
		}
		return cont_state
	}


	pub fn passer<T>(children: &Vec<Node<T>>, board: &T) -> State {
		for child in children {
			if let Error(error) = child.tick(board) { return Error(error); }
		};
		return Pass;
	}

	pub fn inverter<T>(children: &Vec<Node<T>>, board: &T) -> State {
		match children[0].tick(board) {
			Pass => Fail,
			Fail => Pass,
			other => other
		}	
	}
}

fn main () {

}
