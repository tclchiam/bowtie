use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

use itertools::Itertools;

use core::ItemStatus;
use zdd::node::Node;
use zdd::node::NodeId;

pub use self::universe::*;

mod combinations;
mod intersect;
#[cfg(test)]
mod restrict;
mod summarize;
mod union;
mod universe;
mod product;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tree<T: Ord + Hash + Eq> {
    root: NodeId,
    universe: Universe<T>,
}

impl<T: Ord + Hash + Eq> fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.root)
    }
}

impl<T: Clone + Ord + Hash> Tree<T> {
    fn empty(universe: Universe<T>) -> Tree<T> {
        Tree { root: Node::FALSE, universe }
    }

    fn unit(universe: Universe<T>) -> Tree<T> {
        Tree { root: Node::TRUE, universe }
    }

    fn from_root<R>(universe: Universe<T>, root: R) -> Tree<T> where R: Into<NodeId> {
        Tree { root: root.into(), universe }
    }

    pub fn summarize(&self, inclusions: &[T], exclusions: &[T]) -> Vec<ItemStatus<T>> {
        let (items, total) = {
            let mut items = self.universe.items
                .iter()
                .cloned()
                .map(|f| (f, 0))
                .collect::<HashMap<_, _>>();

            let combinations = self.combinations_with(inclusions, exclusions);
            let total = combinations.len();

            combinations.into_iter()
                .flat_map(|f| f)
                .for_each(|item| {
                    *items.entry(item)
                        .or_insert(0) += 1;
                });

            (items, total)
        };
        items.into_iter()
            .map(|(item, count)|
                if count == 0 {
                    ItemStatus::Excluded(item)
                } else if inclusions.contains(&item) {
                    ItemStatus::Selected(item)
                } else if count == total {
                    ItemStatus::Required(item)
                } else {
                    ItemStatus::Available(item)
                })
            .sorted()
    }

    pub fn combinations_recursive(&self) -> BTreeSet<BTreeSet<T>> {
        combinations::combinations_recursive(self.root)
            .into_iter()
            .map(|set| self.universe.get_items(&set))
            .collect::<BTreeSet<_>>()
    }

    pub fn combinations(&self) -> BTreeSet<BTreeSet<T>> {
        combinations::combinations_iter(self.root)
            .into_iter()
            .map(|set| self.universe.get_items(&set))
            .collect::<BTreeSet<_>>()
    }

    pub fn combinations_with(&self, inclusions: &[T], exclusions: &[T]) -> BTreeSet<BTreeSet<T>> {
        let inclusions = inclusions.iter().cloned().collect::<BTreeSet<_>>();
        let exclusions = exclusions.iter().cloned().collect::<BTreeSet<_>>();

        combinations::combinations_iter(self.root)
            .into_iter()
            .map(|set| self.universe.get_items(&set))
            .filter(|set| set.intersection(&inclusions).cloned().collect::<BTreeSet<_>>() == inclusions)
            .filter(|set| set.intersection(&exclusions).collect::<BTreeSet<_>>().is_empty())
            .collect::<BTreeSet<_>>()
    }

    pub fn union(&self, other: &Tree<T>) -> Tree<T> {
        let root = union::union(
            self.root.into(),
            other.root.into());

        Tree::from_root(self.universe.clone(), root)
    }

    pub fn intersect(&self, other: &Tree<T>) -> Tree<T> {
        let root = intersect::intersect(
            self.root.into(),
            other.root.into());

        Tree::from_root(self.universe.clone(), root)
    }

    pub fn product(&self, other: &Tree<T>) -> Tree<T> {
        let root = product::product(
            self.root.into(),
            other.root.into());

        Tree::from_root(self.universe.clone(), root)
    }

    pub fn restrict(&self, inclusions: &[T], exclusions: &[T]) -> Tree<T> {
        let combinations = self.combinations_with(inclusions, exclusions)
            .into_iter()
            .map(|s| s.into_iter().collect_vec())
            .collect_vec();

        self.universe.hyper_tree(&combinations)
    }
}

#[cfg(test)]
mod tests {
    use core::Item;

    use super::Universe;

    #[test]
    fn universe_can_create_empty_tree() {
        let universe: Universe<Item> = Universe::default();
        let empty_tree = universe.empty_tree();

        assert_eq!(
            btreeset!(),
            empty_tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_unit_tree() {
        let universe: Universe<Item> = Universe::default();
        let unit_tree = universe.unit_tree();

        assert_eq!(
            btreeset!(btreeset!()),
            unit_tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_none() {
        let universe: Universe<Item> = Universe::default();
        let tree = universe.tree(&[]);

        assert_eq!(
            universe.unit_tree(),
            tree
        );
        assert_eq!(
            btreeset!(btreeset!()),
            tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_one() {
        let item = Item::new("1");

        let universe = Universe::from(vec![item.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item.clone())),
            universe.tree(&[item]).combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_of_multiple() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone(), item2.clone()]);
        let tree = universe.tree(&[item1.clone(), item2.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone(), item2.clone())),
            tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_an_empty_set_unknown_items_are_ignored() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe: Universe<Item> = Universe::default();
        let tree = universe.tree(&[item1, item2]);

        assert_eq!(
            universe.unit_tree(),
            tree.clone()
        );
        assert_eq!(
            btreeset!(btreeset!()),
            tree.combinations()
        );
    }

    #[test]
    fn universe_can_create_tree_that_represents_a_set_unknown_items_are_ignored() {
        let item1 = Item::new("1");
        let item2 = Item::new("2");

        let universe = Universe::from(vec![item1.clone()]);
        let tree = universe.tree(&[item1.clone(), item2.clone()]);

        assert_eq!(
            btreeset!(btreeset!(item1.clone())),
            tree.combinations()
        )
    }
}
