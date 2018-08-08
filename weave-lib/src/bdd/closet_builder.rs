use bdd::closet::Closet;
use bdd::closet_builder::Error::ConflictingFamilies;
use bdd::closet_builder::Error::ExclusionError;
use bdd::closet_builder::Error::InclusionError;
use bdd::node::Node;
use core::Family;
use core::Item;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ConflictingFamilies(Vec<(Item, Vec<Family>)>),
    InclusionError(Vec<(Family, Vec<Item>)>),
    ExclusionError(Vec<(Family, Vec<Item>)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosetBuilder {
    contents: BTreeMap<Family, Vec<Item>>,
    item_index: BTreeMap<Item, Family>,
    exclusions: BTreeMap<Item, Vec<Item>>,
    inclusions: BTreeMap<Item, Vec<Item>>,
}

impl ClosetBuilder {
    pub fn new() -> ClosetBuilder {
        ClosetBuilder {
            contents: BTreeMap::new(),
            item_index: BTreeMap::new(),
            exclusions: BTreeMap::new(),
            inclusions: BTreeMap::new(),
        }
    }

    pub fn add_item(mut self, family: &Family, item: &Item) -> ClosetBuilder {
        &self.contents.entry(family.clone())
            .or_insert(vec![])
            .push(item.clone());

        &self.item_index.entry(item.clone())
            .or_insert(family.clone());

        self
    }

    pub fn add_exclusion_rule(mut self, selection: &Item, exclusion: &Item) -> ClosetBuilder {
        &self.exclusions.entry(selection.clone())
            .or_insert(vec![])
            .push(exclusion.clone());
        &self.exclusions.entry(exclusion.clone())
            .or_insert(vec![])
            .push(selection.clone());

        self
    }

    pub fn add_inclusion_rule(mut self, selection: &Item, inclusion: &Item) -> ClosetBuilder {
        &self.inclusions.entry(selection.clone())
            .or_insert(vec![])
            .push(inclusion.clone());

        self
    }

    pub fn must_build(self) -> Closet {
        self.build().expect("expected build to return Closet")
    }

    pub fn build(&self) -> Result<Closet, Error> {
        self.validate()?;

        let root = self.contents.iter()
            .map(|(_, items)| items.iter().fold(Node::FALSE_LEAF, |low_branch, item| Node::xor(item, low_branch)))
            .fold(Node::TRUE_LEAF, |low_branch, family_node| low_branch & family_node);

        let item_index = self.item_index.clone();
        let exclusions = self.exclusions.clone();
        Ok(Closet::new(item_index, root, exclusions))
    }

    fn validate(&self) -> Result<(), Error> {
        let conflicts = ClosetBuilder::find_conflicting_families(self);
        if !conflicts.is_empty() {
            return Err(ConflictingFamilies(conflicts));
        }

        let conflicts = ClosetBuilder::find_illegal_include_rules(self);
        if !conflicts.is_empty() {
            return Err(InclusionError(conflicts));
        }

        let conflicts = ClosetBuilder::find_illegal_exclude_rules(self);
        if !conflicts.is_empty() {
            return Err(ExclusionError(conflicts));
        }

        return Ok(());
    }

    fn find_conflicting_families(&self) -> Vec<(Item, Vec<Family>)> {
        self.contents.iter()
            .flat_map(|(family, items)| {
                items.iter()
                    .map(|item| {
                        let item_family = self.item_index
                            .get(item)
                            .expect(&format!("item `{:?}` does not have family", item));

                        if item_family != family {
                            Some((item.clone(), vec![item_family.clone(), family.clone()]))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .filter(|conflict| conflict.is_some())
            .map(|conflict| conflict.unwrap())
            .collect::<Vec<(Item, Vec<Family>)>>()
    }

    fn find_illegal_include_rules(&self) -> Vec<(Family, Vec<Item>)> {
        ClosetBuilder::find_illegal_rules(&self.inclusions, &self.item_index)
    }

    fn find_illegal_exclude_rules(&self) -> Vec<(Family, Vec<Item>)> {
        ClosetBuilder::find_illegal_rules(&self.exclusions, &self.item_index)
    }

    fn find_illegal_rules(rules: &BTreeMap<Item, Vec<Item>>, item_index: &BTreeMap<Item, Family>) -> Vec<(Family, Vec<Item>)> {
        let mut conflicts = rules.iter()
            .flat_map(|(selection, items)| {
                let selection_family = item_index
                    .get(selection)
                    .expect(&format!("item `{:?}` does not have family", selection));

                items.iter()
                    .map(|item| {
                        let item_family = item_index
                            .get(item)
                            .expect(&format!("item `{:?}` does not have family", item));

                        if selection_family == item_family {
                            let mut items = vec![selection.clone(), item.clone()];
                            items.sort();

                            Some((selection_family.clone(), items))
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<_>>()
            })
            .filter(|conflict| conflict.is_some())
            .map(|conflict| conflict.unwrap())
            .collect::<Vec<_>>();

        conflicts.dedup_by(|a, b| a.1 == b.1);
        conflicts
    }
}

#[cfg(test)]
mod no_rules_tests {
    use bdd::node::Node;
    use core::Family;
    use core::Item;
    use super::ClosetBuilder;

    #[test]
    fn two_families_with_one_item_each() {
        let blue = Item::new("blue");
        let jeans = Item::new("jeans");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans);

        let closet = closet_builder.must_build();

        let expected_cousin_node = {
            let high_branch = Node::branch(&blue, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let parent_branch = Node::branch(&jeans, Node::FALSE_LEAF, high_branch);

            parent_branch
        };

        assert_eq!(
            &expected_cousin_node,
            closet.root()
        );


        let both_selected = {
            let closet = closet.select_item(&jeans);
            closet.select_item(&blue)
        };
        assert_eq!(
            &Node::TRUE_LEAF,
            both_selected.root()
        );


        let blue_selected = {
            let closet = closet.select_item(&blue);
            closet.exclude_item(&jeans)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            blue_selected.root()
        );


        let jeans_selected = {
            let closet = closet.select_item(&jeans);
            closet.exclude_item(&blue)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            jeans_selected.root()
        );
    }

    #[test]
    fn one_families_with_two_items() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let shirts = Family::new("shirts");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue);

        let closet = closet_builder.must_build();

        let expected_sibling_node = {
            let low_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let high_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
            let parent_branch = Node::branch(&blue, low_branch, high_branch);

            parent_branch
        };
        assert_eq!(
            &expected_sibling_node,
            closet.root()
        );


        let red_selected = {
            let closet = closet.select_item(&red);
            closet.exclude_item(&blue)
        };
        assert_eq!(
            &Node::TRUE_LEAF,
            red_selected.root()
        );


        let blue_selected = {
            let closet = closet.select_item(&blue);
            closet.exclude_item(&red)
        };
        assert_eq!(
            &Node::TRUE_LEAF,
            blue_selected.root()
        );


        let both_selected = {
            let closet = closet.select_item(&blue);
            closet.select_item(&red)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            both_selected.root()
        );
    }

    #[test]
    fn two_families_with_two_items() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);

        let closet = closet_builder.must_build();

        let blue_false_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let blue_true_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let blue_branch = Node::branch(&blue, blue_false_branch, blue_true_branch);

        let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, blue_branch.clone());
        let slacks_true_branch = Node::branch(&jeans, blue_branch.clone(), Node::FALSE_LEAF);
        let slacks_branch = Node::branch(&slacks, slacks_false_branch.clone(), slacks_true_branch.clone());

        let expected_sibling_node = slacks_branch;
        assert_eq!(
            &expected_sibling_node,
            closet.root()
        );


        let red_selected = {
            let closet = closet.exclude_item(&blue);
            closet.select_item(&red)
        };
        let expected = {
            let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let slacks_true_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

            Node::branch(&slacks, slacks_false_branch, slacks_true_branch)
        };
        assert_eq!(
            &expected,
            red_selected.root()
        );


        let blue_selected = {
            let closet = closet.select_item(&blue);
            closet.exclude_item(&red)
        };
        let expected = {
            let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let slacks_true_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

            Node::branch(&slacks, slacks_false_branch, slacks_true_branch)
        };
        assert_eq!(
            &expected,
            blue_selected.root()
        );


        let both_selected = {
            let closet = closet.select_item(&blue);
            closet.select_item(&red)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            both_selected.root()
        );
    }
}

#[cfg(test)]
mod exclude_rules_tests {
    use bdd::node::Node;
    use core::Family;
    use core::Item;
    use super::ClosetBuilder;

    #[test]
    fn two_families_with_two_items() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &red)
            .add_item(&shirts, &blue)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&red, &jeans);

        let closet = closet_builder.must_build();

        let blue_false_branch = Node::branch(&red, Node::FALSE_LEAF, Node::TRUE_LEAF);
        let blue_true_branch = Node::branch(&red, Node::TRUE_LEAF, Node::FALSE_LEAF);
        let blue_branch = Node::branch(&blue, blue_false_branch, blue_true_branch);

        let blue_branch_red_excluded = Node::branch(&blue, Node::FALSE_LEAF, Node::TRUE_LEAF);

        let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, blue_branch_red_excluded);
        let slacks_true_branch = Node::branch(&jeans, blue_branch.clone(), Node::FALSE_LEAF);
        let slacks_branch = Node::branch(&slacks, slacks_false_branch.clone(), slacks_true_branch.clone());

        let _expected_sibling_node = slacks_branch;
//        assert_eq!(
//            &expected_sibling_node,
//            closet.root()
//        );


        let red_selected = {
            let closet = closet.exclude_item(&blue);
            closet.select_item(&red)
        };
        let expected = Node::branch(&slacks, Node::FALSE_LEAF, Node::TRUE_LEAF);
        assert_eq!(
            &expected,
            red_selected.root()
        );


        let blue_selected = {
            let closet = closet.select_item(&blue);
            closet.exclude_item(&red)
        };
        let expected = {
            let slacks_false_branch = Node::branch(&jeans, Node::FALSE_LEAF, Node::TRUE_LEAF);
            let slacks_true_branch = Node::branch(&jeans, Node::TRUE_LEAF, Node::FALSE_LEAF);

            Node::branch(&slacks, slacks_false_branch, slacks_true_branch)
        };
        assert_eq!(
            &expected,
            blue_selected.root()
        );


        let jeans_selected = {
            let closet = closet.exclude_item(&blue);
            closet.select_item(&jeans)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            jeans_selected.root()
        );


        let both_selected = {
            let closet = closet.select_item(&blue);
            closet.select_item(&red)
        };
        assert_eq!(
            &Node::FALSE_LEAF,
            both_selected.root()
        );
    }
}