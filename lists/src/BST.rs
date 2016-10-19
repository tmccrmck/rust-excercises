use std::mem;

pub struct BST {
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

impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
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
            Link::More(ref mut node) => {
                if elem > node.elem {
                    return BST::insert(&mut BST { root: mem::replace(&mut node.right, Link::Empty) }, elem);
                } else if elem < node.elem {
                    return BST::insert(&mut BST { root: mem::replace(&mut node.left, Link::Empty) }, elem);
                } else {
                    return false;
                }
            }
        }
    }

    pub fn search(&mut self, elem:i32) -> bool {
        match self.root {
            Link::Empty => return false,
            Link::More(ref mut node) => {
                if elem > node.elem {
                    return BST::search(&mut BST { root: mem::replace(&mut node.right, Link::Empty) }, elem);
                } else if elem < node.elem {
                    return BST::search(&mut BST { root: mem::replace(&mut node.left, Link::Empty) }, elem);
                } else {
                    return true;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_insert() {
        let mut bst = BST::new();

        // Check empty bst behaves right
        assert_eq!(bst.insert(1), true);
        assert_eq!(bst.insert(1), false);
        assert_eq!(bst.insert(2), true);
    }

    #[test]
    fn test_search() {
        let mut bst = BST::new();

        // Check empty bst behaves right
        assert_eq!(bst.insert(1), true);
        assert_eq!(bst.search(1), true);
        assert_eq!(bst.insert(2), true);
        assert_eq!(bst.search(3), false);
    }
}