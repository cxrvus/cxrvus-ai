use std::collections::HashMap;

#[derive(Debug)]
enum State { Fail, Pass, Wait, Error(String) }

#[derive(Debug)]
enum Node<T> { Branch(Vec<Node<T>>, fn(&[Node<T>], &T) -> State), Leaf(fn(&T) -> State) }

enum Value { String(String), Number(i32), Bool(bool) }

type BlackBoard = HashMap<String, Value>;

impl<T> Node<T> {
    fn tick(&self, board: &T) -> State {
        match self {
            Node::Branch(children, func) => func(children.as_slice(), board),
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
}

mod branch_funcs {
	use super::*;

	pub fn sequence<T>(children: &[Node<T>], board: &T) -> State {
		for child in children {
			match child.tick(board) {
				State::Pass => continue,
				other => return other,
			}
		}
		State::Pass
	}

	pub fn fallback<T>(children: &[Node<T>], board: &T) -> State {
		for child in children {
			match child.tick(board) {
				State::Fail => continue,
				other => return other,
			}
		}
		State::Fail
	}
}
