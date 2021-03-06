#![allow(dead_code)]

enum Formula {
	True, False,
	And(Formula, Formula), Or(Formula, Formula),
}
impl Formula {
	fn resolve(&self) -> bool {
		match self {
			&Formula::True => true,
			&Formula::False => false,
			&Formula::And(left, right) => left.resolve() && right.resolve(),
			&Formula::Or(left, right) => left.resolve() || right.resolve(),
		}
	}
}

fn and(left: Formula, right: Formula) -> Formula {
	Formula::False
}
fn or(left: Formula, right: Formula) -> Formula {
	Formula::True
}

mod formula_should {
	use super::{Formula::{self, True, False}, and, or};

	fn assert_resolve(expected: bool, f: Formula) {
		assert_eq!(expected, f.resolve());
	}

	#[test]
	fn resolve_true_when_true() {
		assert_resolve(true, True);
	}

	#[test]
	fn resolve_true_when_true_and_true() {
		assert_resolve(true, and(True, True));
	}

	#[test]
	fn resolve_false_when_true_and_false() {
		assert_resolve(false, and(True, False));
	}

	#[test]
	fn resolve_false_when_false_or_false() {
		assert_resolve(false, or(False, False));
	}

	#[test]
	fn resolve_true_when_true_or_false() {
		assert_resolve(true, or(True, False));
	}
}

/// # spy
/// Implements a very simple spy which counts application calls.
mod spy {
	#[derive(Default,Clone,Copy)]
	pub struct Stats {
		hello: u32,
		goodbye: u32,
	}

	pub struct Api {
		stats: Stats,
	}

	impl Api {
		pub fn new() -> Self {
			Api { stats: Stats::default(), }
		}

		pub fn stats(&self) -> Stats {
			self.stats
		}

		pub fn hello(&self) {
		}
		pub fn goodbye(&self) {
		}
	}

	impl Stats {
		pub fn hello(&self) -> u32 {
			self.hello
		}

		pub fn goodbye(&self) -> u32 {
			self.goodbye
		}
	}

	mod should {

		use super::Api;
		
		fn assert_stats(hello: u32, goodbye: u32, api: Api) {
			let stats = api.stats();
			assert_eq!(hello, stats.hello(), "hello");
			assert_eq!(goodbye, stats.goodbye(), "goodbye");
		}

		#[test]
		fn register_hello_0_goodbye_0_after_init() {
			let api = Api::new();

			assert_stats(0, 0, api);
		}

		#[test]
		fn register_hello_1_goodbye_0_after_hello() {
			let api = Api::new();
			api.hello();

			assert_stats(1, 0, api);
		}

		#[test]
		fn register_hello_0_goodbye_1_after_goodbye() {
			let api = Api::new();
			api.goodbye();

			assert_stats(0, 1, api);
		}

		#[test]
		fn register_hello_1_goodbye_1_after_hello_goodbye() {
			let api = Api::new();
			api.hello();
			api.goodbye();

			assert_stats(1, 1, api);
		}

		#[test]
		fn register_hello_2_goodbye_0_after_hello_hello() {
			let api = Api::new();
			api.hello();
			api.hello();

			assert_stats(2, 0, api);
		}
	}
}

/// Optional
/// Try to implement a Tree structure. A template is provided but it's not working
mod tree {
	pub struct Tree<T> {
		parent: Option<Tree<T>>,
		value: T,
		children: Vec<TreeNode<T>>,
	}


	impl<T: Copy> Tree<T> {
		pub fn new(value: T) -> Self {
			Tree { parent: None, value, children: vec![] }
		}

		pub fn parent(&self) -> Option<Self> {
			self.parent
		}

		pub fn value(&self) -> T {
			self.value
		}

		pub fn values_from_root(&self) -> Vec<T> {
			let mut values = vec![];
			values.push(self.value());
			let mut current = self.parent();
			while let Some(tree) = current {
				values.push(tree.value());
				current = tree.parent();
			}
			values.reverse();
			values
		}

		pub fn push(&mut self, value: T) -> Self {
			let child = Tree { parent: *self, value, children: vec![] };
			self.children.push(child);
			child
		}

		fn collect(&self, values: &mut Vec<T>) {
			values.push(self.value);
			for child in self.children {
				child.collect(values);
			}
		}

		pub fn to_vec(&self) -> Vec<T> {
			let mut values = vec![];
			self.collect(&mut values);
			values
		}
	}
}

mod tree_should {
	use super::tree::Tree;

	#[test]
	fn contains_1() {
		let tree = Tree::new("1");
		assert_eq!(vec!["1"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a() {
		let mut tree = Tree::new("1");
		tree.push("1a");
		assert_eq!(vec!["1", "1a"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a_1b() {
		let mut tree = Tree::new("1");
		tree.push("1a");
		tree.push("1b");
		assert_eq!(vec!["1", "1a", "1b"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a_1a1_1b() {
		let mut tree = Tree::new("1");
		let mut t1a = tree.push("1a");
		t1a.push("1a1");
		tree.push("1b");
		assert_eq!(vec!["1", "1a", "1a1", "1b"], tree.to_vec());
	}

	#[test]
	fn contains_1_1a_1a1_1b_1b1() {
		let mut tree = Tree::new("1");
		let mut t1a = tree.push("1a");
		t1a.push("1a1");
		let mut t1b = tree.push("1b");
		t1b.push("1b1");
		assert_eq!(vec!["1", "1a", "1a1", "1b", "1b1"], tree.to_vec());
	}

	#[test]
	fn have_no_more_parent_after_drop() {
		let mut tree = Tree::new("1");
		let mut t1a = tree.push("1a");
		let t1a1 = t1a.push("1a1");
		let mut t1b = tree.push("1b");
		let t1b1 = t1b.push("1b1");

		assert_eq!(vec!["1", "1a", "1a1"], t1a1.values_from_root());
		assert_eq!(vec!["1", "1b", "1b1"], t1b1.values_from_root());

		drop(tree);

		assert_eq!(vec!["1a", "1a1"], t1a1.values_from_root());
		assert_eq!(vec!["1b", "1b1"], t1b1.values_from_root());
	}
}
