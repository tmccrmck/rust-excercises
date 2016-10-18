pub struct Bst {
    root: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl Bst {
	pub fn new() -> Self {
		Bst { root: Link::Empty }
	}

	pub fn insert(&mut self, elem: i32) -> bool {
		match self.root {
			Link::Empty => {
				let new_node = Box::new(Node {
		            elem: elem,
		            left: Link::Empty,
		            right: Link::Empty,
		        });
				self.root = Link::More(new_node);
				return true;
			},
			Link::More(ref node) => {
				if elem == node.elem {
					return false;
				} else if elem > node.elem { 
					Bst::insert(&mut Bst { root: node.right }, elem);
				} else if elem < node.elem {
					Bst::insert(&mut Bst { root: node.left }, elem);
				}

			}
		}
		return true;
	}
}