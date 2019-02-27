use std::fmt::Debug;

use weave::Forest;

pub fn both_trees_are_empty<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::empty();
    let tree2 = F::empty();

    let expected = F::empty();

    (tree1, tree2, expected)
}

pub fn left_is_empty_right_is_unit<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::empty();
    let tree2 = F::unit(&["1", "2"]);

    let expected = F::empty();

    (tree1, tree2, expected)
}

pub fn left_is_empty_right_is_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::empty();
    let tree2 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = F::empty();

    (tree1, tree2, expected)
}

pub fn left_is_unit_right_is_empty<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::unit(&["1", "2"]);
    let tree2 = F::empty();

    let expected = F::empty();

    (tree1, tree2, expected)
}

pub fn trees_are_equal_unit<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::unit(&["1", "2"]);
    let tree2 = F::unit(&["1", "2"]);

    let expected = F::unit(&["1", "2"]);

    (tree1, tree2, expected)
}

pub fn trees_are_disjoint_units<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::unit(&["1", "2"]);
    let tree2 = F::unit(&["2", "3"]);

    let expected = F::empty();

    (tree1, tree2, expected)
}

pub fn left_is_unit_right_is_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::unit(&["1", "2"]);
    let tree2 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = F::unit(&["1", "2"]);

    (tree1, tree2, expected)
}

pub fn trees_are_equal_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);
    let tree2 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    let expected = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);

    (tree1, tree2, expected)
}

pub fn trees_are_disjoint_many<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"]
    ]);
    let tree2 = F::many(&[
        vec!["1", "3"],
        vec!["2", "4"]
    ]);

    let expected = F::empty();

    (tree1, tree2, expected)
}

pub fn trees_are_have_single_commonality<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"],
    ]);
    let tree2 = F::many(&[
        vec!["2", "3"],
        vec!["3", "4"],
        vec!["4", "5"],
    ]);

    let expected = F::unit(&["2", "3"]);

    (tree1, tree2, expected)
}

pub fn trees_are_have_multiple_commonality<'a, F: Forest<&'a str> + Debug + Eq + Clone>() -> (F, F, F) {
    let tree1 = F::many(&[
        vec!["1", "2"],
        vec!["2", "3"],
        vec!["3", "4"],
    ]);
    let tree2 = F::many(&[
        vec!["2", "3"],
        vec!["3", "4"],
        vec!["4", "5"],
    ]);

    let expected = F::many(&[
        vec!["2", "3"],
        vec!["3", "4"],
    ]);

    (tree1, tree2, expected)
}