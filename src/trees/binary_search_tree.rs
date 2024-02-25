#[derive(Debug, Clone, PartialEq)]
pub enum Tree<A> {
    Node(A, Box<Tree<A>>, Box<Tree<A>>),
    Leaf,
}

impl<A: PartialOrd> Tree<A> {
    pub fn new() -> Self {
        Tree::Leaf
    }

    pub fn insert(self, node: A) -> Self {
        match self {
            Tree::Leaf => Tree::Node(node, Box::new(Tree::Leaf), Box::new(Tree::Leaf)),
            Tree::Node(value, left, right) => {
                if node > value {
                    Tree::Node(value, left, Box::new(right.insert(node)))
                } else {
                    Tree::Node(value, Box::new(left.insert(node)), right)
                }
            }
        }
    }
}

#[test]
fn should_create_binary_search_tree() {
    let arr: Vec<i32> = vec![4, 1, 3, 5, 2, 8, 6, 7];

    let mut tree = Tree::new() as Tree<i32>;

    for i in arr {
        tree = tree.insert(i);
    }

    println!("{:?}", tree);

    assert_eq!(
        tree,
        Tree::Node(
            4,
            Box::new(
                Tree::Node(
                    1,
                    Box::new(Tree::Leaf),
                    Box::new(
                        Tree::Node(
                            3,
                            Box::new(
                                Tree::Node(
                                    2,
                                    Box::new(Tree::Leaf),
                                    Box::new(Tree::Leaf)
                                )
                            ),
                            Box::new(Tree::Leaf)
                        )
                    )
                )
            ),
            Box::new(
                Tree::Node(
                    5,
                    Box::new(Tree::Leaf),
                    Box::new(
                        Tree::Node(
                            8,
                            Box::new(
                                Tree::Node(
                                    6,
                                    Box::new(Tree::Leaf),
                                    Box::new(
                                        Tree::Node(
                                            7,
                                            Box::new(Tree::Leaf),
                                            Box::new(Tree::Leaf)
                                        )
                                    )
                                )
                            ),
                            Box::new(Tree::Leaf)
                        )
                    )
                )
            )
        )
    );
}
