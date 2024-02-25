#[derive(Debug, PartialEq, Eq)]
pub enum Tree<A> {
    Leaf(A),
    Node(Box<Tree<A>>, Box<Tree<A>>),
}

impl Tree<i32> {
    pub fn new(values: Vec<i32>) -> Self {
        match values.len() {
            0 => panic!("Invalid parameter"),
            1 => Tree::Leaf(values[0]),
            n => {
                let mid = n / 2;
                Tree::Node(
                    Box::new(
                        Tree::new(
                            values[0..mid].to_vec()
                        )
                    ),
                    Box::new(
                        Tree::new(
                            values[mid..n].to_vec()
                        )
                    )
                )
            }
        }
    }
}

#[test]
fn should_create_binary_tree() {
    let tree = Tree::new(vec![1, 2, 3, 4]);
    println!("{:?}", tree);

    assert_eq!(
        tree,
        Tree::Node(
            Box::new(
                Tree::Node(
                    Box::new(
                        Tree::Leaf(1)
                    ),
                    Box::new(
                        Tree::Leaf(2)
                    )
                )
            ),
            Box::new(
                Tree::Node(
                    Box::new(
                        Tree::Leaf(3)
                    ),
                    Box::new(
                        Tree::Leaf(4)
                    )
                )
            )
        )
    )
}

#[test]
fn should_create_big_binary_tree() {
    let tree = Tree::new(vec![
        1, 2, 3, 4, 5, 6, 7, 8,
        9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32
    ]);
    println!("{:?}", tree);

    assert_eq!(
        tree,
        Tree::Node(
            Box::new(
                Tree::Node(
                    Box::new(
                        Tree::Node(
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(1)
                                            ),
                                            Box::new(
                                                Tree::Leaf(2)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(3)
                                            ),
                                            Box::new(
                                                Tree::Leaf(4)
                                            )
                                        )
                                    )
                                )
                            ),
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(5)
                                            ),
                                            Box::new(
                                                Tree::Leaf(6)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(7)
                                            ),
                                            Box::new(
                                                Tree::Leaf(8)
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    ),
                    Box::new(
                        Tree::Node(
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(9)
                                            ),
                                            Box::new(
                                                Tree::Leaf(10)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(11)
                                            ),
                                            Box::new(
                                                Tree::Leaf(12)
                                            )
                                        )
                                    )
                                )
                            ),
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(13)
                                            ),
                                            Box::new(
                                                Tree::Leaf(14)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(15)
                                            ),
                                            Box::new(
                                                Tree::Leaf(16)
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            ),
            Box::new(
                Tree::Node(
                    Box::new(
                        Tree::Node(
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(17)
                                            ),
                                            Box::new(
                                                Tree::Leaf(18)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(19)
                                            ),
                                            Box::new(
                                                Tree::Leaf(20)
                                            )
                                        )
                                    )
                                )
                            ),
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(21)
                                            ),
                                            Box::new(
                                                Tree::Leaf(22)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(23)
                                            ),
                                            Box::new(
                                                Tree::Leaf(24)
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    ),
                    Box::new(
                        Tree::Node(
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(25)
                                            ),
                                            Box::new(
                                                Tree::Leaf(26)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(27)
                                            ),
                                            Box::new(
                                                Tree::Leaf(28)
                                            )
                                        )
                                    )
                                )
                            ),
                            Box::new(
                                Tree::Node(
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(29)
                                            ),
                                            Box::new(
                                                Tree::Leaf(30)
                                            )
                                        )
                                    ),
                                    Box::new(
                                        Tree::Node(
                                            Box::new(
                                                Tree::Leaf(31)
                                            ),
                                            Box::new(
                                                Tree::Leaf(32)
                                            )
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            )
        )
    )
}
