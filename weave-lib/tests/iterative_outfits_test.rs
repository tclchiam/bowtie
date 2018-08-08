extern crate weave_lib;

#[cfg(test)]
mod no_rules_tests {
    use std::collections::BTreeMap;
    use weave_lib::core::Family;
    use weave_lib::core::Item;
    use weave_lib::core::Outfit;
    use weave_lib::core::OutfitError::Validation;
    use weave_lib::core::ValidationError::MultipleItemsPerFamily;
    use weave_lib::core::ValidationError::UnknownItems;
    use weave_lib::iterative::closet_builder::ClosetBuilder;
    use weave_lib::iterative::outfits::complete_outfit;

    #[test]
    fn no_rules_no_selections() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans, blue]));
        assert_eq!(
            expected,
            complete_outfit(closet, vec![])
        );
    }

    #[test]
    fn no_rules_one_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans, red.clone()]));
        assert_eq!(
            expected,
            complete_outfit(closet, vec![red])
        );
    }

    #[test]
    fn no_rules_selection_for_each_family() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![slacks.clone(), blue.clone()]));
        assert_eq!(
            expected,
            complete_outfit(closet, vec![slacks, blue])
        );
    }

    #[test]
    fn no_rules_unknown_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");
        let black = Item::new("black");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Err(Validation(UnknownItems(vec![black.clone()])));
        assert_eq!(
            expected,
            complete_outfit(closet, vec![jeans, black])
        );
    }

    #[test]
    fn no_rules_more_selections_than_families() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = {
            let mut duplicates = BTreeMap::new();
            duplicates.insert(pants, vec![jeans.clone(), slacks.clone()]);

            Err(Validation(MultipleItemsPerFamily(duplicates)))
        };

        assert_eq!(
            expected,
            complete_outfit(closet, vec![jeans, blue, slacks])
        );
    }
}

#[cfg(test)]
mod exclusion_rules_tests {
    use weave_lib::core::Family;
    use weave_lib::core::Item;
    use weave_lib::core::Outfit;
    use weave_lib::core::OutfitError::Validation;
    use weave_lib::core::ValidationError::ConflictingItems;
    use weave_lib::iterative::closet_builder::ClosetBuilder;
    use weave_lib::iterative::outfits::complete_outfit;

    #[test]
    fn exclusion_rule_with_one_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&blue, &jeans);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![slacks, blue.clone()]));
        assert_eq!(
            expected,
            complete_outfit(closet.clone(), vec![blue])
        );

        let expected = Ok(Outfit::new(vec![jeans.clone(), red]));
        assert_eq!(
            expected,
            complete_outfit(closet.clone(), vec![jeans])
        );
    }

    #[test]
    fn exclusion_rule_with_conflicting_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&blue, &jeans);
        let closet = closet_builder.must_build();

        let expected = Err(Validation(ConflictingItems(vec![jeans.clone(), blue.clone()])));
        assert_eq!(
            expected,
            complete_outfit(closet, vec![blue, jeans])
        );
    }

    #[test]
    #[should_panic]
    fn exclusion_rules_with_impossible_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_exclusion_rule(&blue, &jeans)
            .add_exclusion_rule(&blue, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![blue.clone()]));
        assert_eq!(
            expected,
            complete_outfit(closet.clone(), vec![blue])
        );
    }
}

#[cfg(test)]
mod inclusion_rules_tests {
    use weave_lib::core::Family;
    use weave_lib::core::Item;
    use weave_lib::core::Outfit;
    use weave_lib::iterative::closet_builder::ClosetBuilder;
    use weave_lib::iterative::outfits::complete_outfit;

    #[test]
    fn inclusion_rule_with_one_selection() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans.clone(), blue.clone()]));
        assert_eq!(
            expected,
            complete_outfit(closet.clone(), vec![])
        );


        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_inclusion_rule(&jeans, &red);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![jeans, red]));
        assert_eq!(
            expected,
            complete_outfit(closet.clone(), vec![])
        );
    }

    #[test]
    fn inclusion_rule_is_one_way() {
        let blue = Item::new("blue");
        let red = Item::new("red");

        let jeans = Item::new("jeans");
        let slacks = Item::new("slacks");

        let shirts = Family::new("shirts");
        let pants = Family::new("pants");

        let closet_builder = ClosetBuilder::new()
            .add_item(&shirts, &blue)
            .add_item(&shirts, &red)
            .add_item(&pants, &jeans)
            .add_item(&pants, &slacks)
            .add_inclusion_rule(&red, &slacks);
        let closet = closet_builder.must_build();

        let expected = Ok(Outfit::new(vec![slacks.clone(), blue]));
        assert_eq!(
            expected,
            complete_outfit(closet.clone(), vec![slacks])
        );
    }
}