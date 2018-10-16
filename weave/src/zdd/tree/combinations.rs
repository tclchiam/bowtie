use zdd::node::Node;
use zdd::node::NodeId;
use zdd::node::Priority;

pub fn combinations(root: NodeId) -> Vec<Vec<Priority>> {
    combinations_inner(root, &[])
        .unwrap_or_else(Vec::new)
}

fn combinations_inner(root: NodeId, path: &[Priority]) -> Option<Vec<Vec<Priority>>> {
    match Node::from(root) {
        Node::Branch(id, low, high) => {
            let low = combinations_inner(low, &path);

            let path = {
                let mut path = path.to_vec();
                path.push(id);
                path
            };

            let high = combinations_inner(high, &path);

            let vec = vec![low, high]
                .into_iter()
                .filter_map(|f| f)
                .flatten()
                .collect();

            Some(vec)
        }
        Node::Leaf(true) => Some(vec![path.to_vec()]),
        Node::Leaf(false) => None,
    }
}

#[cfg(test)]
mod tests {
    use core::Item;
    use zdd::Universe;

    #[test]
    fn tree_with_two_sets_with_no_overlap() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);

        let tree = universe.hyper_tree(&[
            vec![item1.clone()],
            vec![item2.clone()]
        ]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone()), btreeset!(item2.clone())),
            tree.combinations()
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.offset(&btreeset![item2])
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.onset(&btreeset![item1])
        );
    }

    #[test]
    fn tree_with_two_sets_with_one_overlap() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");
        let item3 = Item::new("3");

        let universe = Universe::from(vec![item1.clone(), item2.clone(), item3.clone()]);

        let tree = universe.hyper_tree(&[
            vec![item1.clone(), item2.clone()],
            vec![item2.clone(), item3.clone()]
        ]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone()), btreeset!(item2.clone(), item3.clone())),
            tree.combinations()
        );

        assert_eq!(
            btreeset!(),
            tree.offset(&btreeset![item2.clone()])
        );
        assert_eq!(
            btreeset!(btreeset!(item2.clone(), item3.clone())),
            tree.offset(&btreeset![item1.clone()])
        );

        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone())),
            tree.onset(&btreeset![item1])
        );
    }
}