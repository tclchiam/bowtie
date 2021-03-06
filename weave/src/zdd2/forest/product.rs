#[cfg(test)]
mod tests {
    use super::super::Forest;

    #[test]
    fn empty_right_returns_empty() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::<&str>::empty();

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn empty_left_returns_empty() {
        let forest1 = Forest::<&str>::empty();
        let forest2 = Forest::unit(&["1"]);

        assert_eq!(
            Forest::<&str>::empty(),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn unit_forests_with_overlap_returns_many() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::unit(&["1"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn disjoint_unit_forests_returns_many_1() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::unit(&["3"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn disjoint_unit_forests_returns_unit() {
        let forest1 = Forest::unit(&["1", "2"]);
        let forest2 = Forest::unit(&["3", "4"]);

        assert_eq!(
            Forest::unit(&["1", "2", "3", "4"]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_single_unit_forest_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = Forest::unit(&["4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "4"],
                vec!["2", "3", "4"]
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_double_unit_forest_with_overlap_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = Forest::unit(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3", "4"],
                vec!["2", "3", "4"]
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_unique_forest_with_overlap_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["2", "3"]
        ]);
        let forest2 = Forest::unique(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
                vec!["2", "3"],
                vec!["2", "3", "4"]
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_unique_forest_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["5", "6"]
        ]);
        let forest2 = Forest::unique(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3"],
                vec!["1", "2", "4"],
                vec!["5", "6", "3"],
                vec!["5", "6", "4"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forest_and_double_unit_forest_returns_many() {
        let forest1 = Forest::many(&[
            vec!["1", "2"],
            vec!["5", "6"]
        ]);
        let forest2 = Forest::unit(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "2", "3", "4"],
                vec!["5", "6", "3", "4"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forests_returns_many_1() {
        let forest1 = Forest::unique(&["1", "2"]);
        let forest2 = Forest::unique(&["3", "4"]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "3"],
                vec!["1", "4"],
                vec!["2", "3"],
                vec!["2", "4"],
            ]),
            Forest::product(forest1, forest2)
        );
    }

    #[test]
    fn many_forests_returns_many_2() {
        let forest1 = Forest::unique(&["1", "2"]);
        let forest2 = Forest::many(&[
            vec!["3", "4"],
            vec!["7", "8"]
        ]);

        assert_eq!(
            Forest::many(&[
                vec!["1", "3", "4"],
                vec!["1", "7", "8"],
                vec!["2", "3", "4"],
                vec!["2", "7", "8"],
            ]),
            Forest::product(forest1, forest2)
        );
    }
}