use bdd::closet::complete_outfit::complete_outfit;
use bdd::node::Node;
use core::Family;
use core::Item;
use core::Outfit;
use core::OutfitError;
use std::collections::BTreeMap;

mod categorize;
mod complete_outfit;
mod node_count;

#[derive(Debug, Clone, PartialEq)]
pub struct Closet {
    item_index: BTreeMap<Item, Family>,
    root: Node,
}

impl Closet {
    pub fn new(
        item_index: BTreeMap<Item, Family>,
        root: Node,
    ) -> Closet {
        Closet { item_index, root }
    }

    pub fn get_family(&self, item: &Item) -> Option<&Family> {
        self.item_index.get(item).map(|family| family)
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn select_item(&self, item: &Item) -> Closet {
        let new_root = Node::restrict(&self.root, item, true);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
        }
    }

    pub fn exclude_item(&self, item: &Item) -> Closet {
        let new_root = Node::restrict(&self.root, item, false);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
        }
    }

    pub fn reduce(&self) -> Closet {
        let new_root = Node::reduce(&self.root);

        Closet {
            item_index: self.item_index.clone(),
            root: new_root,
        }
    }

    pub fn complete_outfit(&self, selections: Vec<Item>) -> Result<Outfit, OutfitError> {
        complete_outfit(self, selections)
    }
}
