// Advent of Code 2020
// Day 7

use std::collections::{HashMap, HashSet};
use std::str::Split;

use std::fs;

// Test the examples from the puzzle to a single depth,
// i.e. test that we can correctly parse a single rule and identify what a given bag
// immediately contains.
#[cfg(test)]
mod test_examples_single_depth {
    use super::*;

    #[test]
    fn test_light_red() {
        let rules = "
        light red bags contain 1 bright white bag, 2 muted yellow bags.";

        let rules = Ruleset::from_str(rules);

        let light_red = Bag::from_str("light red");

        let contains = light_red.contains_directly(&rules);

        let bright_white = contains.get(&Bag::from_str("bright white")).expect("light red does not contain bright white");
        let muted_yellow = contains.get(&Bag::from_str("muted yellow")).expect("light red does not contain muted yellow");

        assert_eq!(1, *bright_white);
        assert_eq!(2, *muted_yellow);
    }

    #[test]
    fn test_dark_orange() {
        let rules = "
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.";

        let rules = Ruleset::from_str(rules);

        let light_red = Bag::from_str("dark orange");

        let contains = light_red.contains_directly(&rules);

        let bright_white = contains.get(&Bag::from_str("bright white")).expect("dark orange does not contain bright white");
        let muted_yellow = contains.get(&Bag::from_str("muted yellow")).expect("dark orange does not contain muted yellow");

        assert_eq!(3, *bright_white);
        assert_eq!(4, *muted_yellow);
    }

    #[test]
    fn test_bright_white() {
        let rules = "
        bright white bags contain 1 shiny gold bag.";

        let rules = Ruleset::from_str(rules);

        let bright_white = Bag::from_str("bright white");

        let contains = bright_white.contains_directly(&rules);

        let shiny_gold = contains.get(&Bag::from_str("shiny gold")).expect("bright white does not contain shiny gold");

        assert_eq!(1, *shiny_gold);
    }

    #[test]
    fn test_muted_yellow() {
        let rules = "
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

        let rules = Ruleset::from_str(rules);

        let muted_yellow = Bag::from_str("muted yellow");

        let contains = muted_yellow.contains_directly(&rules);

        let shiny_gold = contains.get(&Bag::from_str("shiny gold")).expect("muted yellow does not contain shiny gold");
        let faded_blue = contains.get(&Bag::from_str("faded blue")).expect("muted yellow does not contain faded blue");

        assert_eq!(2, *shiny_gold);
        assert_eq!(9, *faded_blue);
    }

    #[test]
    fn test_shiny_gold() {
        let rules = "
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.";

        let rules = Ruleset::from_str(rules);

        let shiny_gold = Bag::from_str("shiny gold");

        let contains = shiny_gold.contains_directly(&rules);

        let dark_olive = contains.get(&Bag::from_str("dark olive")).expect("shiny gold does not contain dark olive");
        let vibrant_plum = contains.get(&Bag::from_str("vibrant plum")).expect("shiny gold does not contain vibrant plum");

        assert_eq!(1, *dark_olive);
        assert_eq!(2, *vibrant_plum);
    }

    #[test]
    fn test_dark_olive() {
        let rules = "
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.";

        let rules = Ruleset::from_str(rules);

        let dark_olive = Bag::from_str("dark olive");

        let contains = dark_olive.contains_directly(&rules);

        let faded_blue = contains.get(&Bag::from_str("faded blue")).expect("dark olive does not contain faded blue");
        let dotted_black = contains.get(&Bag::from_str("dotted black")).expect("dark olive does not contain dotted black");

        assert_eq!(3, *faded_blue);
        assert_eq!(4, *dotted_black);
    }

    #[test]
    fn test_vibrant_plum() {
        let rules = "
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";

        let rules = Ruleset::from_str(rules);

        let vibrant_plum = Bag::from_str("vibrant plum");

        let contains = vibrant_plum.contains_directly(&rules);

        let faded_blue = contains.get(&Bag::from_str("faded blue")).expect("vibrant plum does not contain faded blue");
        let dotted_black = contains.get(&Bag::from_str("dotted black")).expect("vibrant plum does not contain dotted black");

        assert_eq!(5, *faded_blue);
        assert_eq!(6, *dotted_black);
    }

    #[test]
    fn test_faded_blue() {
        let rules = "
        faded blue bags contain no other bags.";

        let rules = Ruleset::from_str(rules);

        let faded_blue = Bag::from_str("faded blue");

        let contains = faded_blue.contains_directly(&rules);

        assert_eq!(0, contains.len());
    }

    #[test]
    fn test_dotted_black() {
        let rules = "
        dotted black bags contain no other bags.";

        let rules = Ruleset::from_str(rules);

        let dotted_black = Bag::from_str("dotted black");

        let contains = dotted_black.contains_directly(&rules);

        assert_eq!(0, contains.len());
    }
}

#[cfg(test)]
mod test_example {
    use super::*;

    // Tests that we can collect all the types of bags
    // that exist in a ruleset.
    #[test]
    fn example_collect() {
        let rules = "
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let ruleset = Ruleset::from_str(rules);
        let bags = ruleset.bags();

        assert_eq!(true, bags.contains(&Bag::from_str("light red")));
        assert_eq!(true, bags.contains(&Bag::from_str("dark orange")));
        assert_eq!(true, bags.contains(&Bag::from_str("bright white")));
        assert_eq!(true, bags.contains(&Bag::from_str("muted yellow")));
        assert_eq!(true, bags.contains(&Bag::from_str("shiny gold")));
        assert_eq!(true, bags.contains(&Bag::from_str("dark olive")));
        assert_eq!(true, bags.contains(&Bag::from_str("vibrant plum")));
        assert_eq!(true, bags.contains(&Bag::from_str("faded blue")));
        assert_eq!(true, bags.contains(&Bag::from_str("dotted black")));

        assert_eq!(false, bags.contains(&Bag::from_str("shiny indigo")));
        assert_eq!(false, bags.contains(&Bag::from_str("fluorescent turqoise")));
        assert_eq!(false, bags.contains(&Bag::from_str("posh cream")));
    }

    // Test the complete example from part 1, positive cases.
    #[test]
    fn example_part1_positive() {
        let rules = "
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let rules = Ruleset::from_str(rules);

        let shiny_gold = Bag::from_str("shiny gold");

        let light_red = Bag::from_str("light red");
        assert_eq!(true, light_red.contains(&rules, &shiny_gold));

        let bright_white = Bag::from_str("bright white");
        assert_eq!(true, bright_white.contains(&rules, &shiny_gold));

        let muted_yellow = Bag::from_str("muted yellow");
        assert_eq!(true, muted_yellow.contains(&rules, &shiny_gold));

        let dark_orange = Bag::from_str("dark orange");
        assert_eq!(true, dark_orange.contains(&rules, &shiny_gold));
    }

    // Test the complete example from part 1, positive cases.
    #[test]
    fn example_part1_negative() {
        let rules = "
        light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let rules = Ruleset::from_str(rules);

        let shiny_gold = Bag::from_str("shiny gold");

        assert_eq!(false, shiny_gold.contains(&rules, &shiny_gold));

        let dark_olive = Bag::from_str("dark olive");
        assert_eq!(false, dark_olive.contains(&rules, &shiny_gold));

        let vibrant_plum = Bag::from_str("vibrant plum");
        assert_eq!(false, vibrant_plum.contains(&rules, &shiny_gold));

        let faded_blue = Bag::from_str("faded blue");
        assert_eq!(false, faded_blue.contains(&rules, &shiny_gold));

        let dotted_black = Bag::from_str("dotted black");
        assert_eq!(false, dotted_black.contains(&rules, &shiny_gold));
    }
}

#[cfg(test)]
mod test_bag {
    use super::*;

    #[test]
    fn test_from_string() {
        let bag = Bag::from_str("bright red");
        assert_eq!("bright red", bag.colour());
    }

    #[test]
    fn test_from_iter() {
        let colour = "bright red";
        let bag = Bag::from_iter(&mut colour.split(" "));
        assert_eq!("bright red", bag.colour());
    }
}

#[cfg(test)]
mod test_puzzles {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(208, part1());
    }
}

struct Ruleset {
    ruleset: HashMap<Bag, HashMap<Bag, u32>>
}

impl Ruleset {
    fn from_str(rules: &str) -> Ruleset {
        let mut ruleset: HashMap<Bag, HashMap<Bag, u32>> = HashMap::new();

        for rule in rules.split('\n') {
            let rule = rule.trim();

            // Skip blank lines.
            if rule.is_empty() {
                continue;
            }

            let mut rule = rule.split(" ");

            let containing_bag = Bag::from_iter(&mut rule);

            // Now we should have "bags contain"
            if rule.next().unwrap() != "bags" {
                panic!("Invalid rule.");
            }

            if rule.next().unwrap() != "contain" {
                panic!("Invalid rule.");
            }

            // Now loop over the remaining part of the rule,
            // to get all the kinds of bag that this bag can contain.
            let mut contains: HashMap<Bag, u32> = HashMap::new();

            loop {
                // First, a number.
                let number = rule.next().unwrap();

                // Is this actually a number? It could also be the word "no"
                if number == "no" {
                    if rule.next().unwrap() != "other" {
                        panic!("Invalid rule.");
                    }

                    if rule.next().unwrap() != "bags." {
                        panic!("Invalid rule.");
                    }

                    break;
                }

                // Otherwise it _should_ be a number.
                // If it's not, this will panic.
                let number = number.parse().unwrap();

                // Now, a bag.
                let bag = Bag::from_iter(&mut rule);

                // Put the bag in the bag.
                contains.insert(bag, number);


                // Now "bag." or "bags." if this is the last bag,
                // or "bag," or "bags," if not.
                // For anything else we panic.
                match rule.next().unwrap() {
                    "bag." | "bags." => break,
                    "bag," | "bags," => continue,
                    _ => panic!("Invalid rule")
                }
            }

            ruleset.insert(containing_bag, contains);
        }

        return Ruleset {
            ruleset
        };
    }

    // Returns all the bag colours defined under this ruleset.
    fn bags(&self) -> HashSet<&Bag> {
        let mut s: HashSet<&Bag> = HashSet::new();

        for (bag, _) in &self.ruleset {
            s.insert(bag);
        }

        return s;
    }

    // Returns the types and number of bags that can be contained
    // in the given bag, with this ruleset.
    fn contains(&self, bag: &Bag) -> &HashMap<Bag, u32> {
        return self.ruleset.get(bag).expect(&format!("Bag {} not defined in this ruleset", bag.colour()));
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Bag {
    colour: String
}

impl Bag {
    fn from_str(s: &str) -> Bag {
        return Bag {
            colour: String::from(s)
        }
    }

    fn from_iter(s: &mut Split<&str>) -> Bag {
        let mut b = String::new();
        b.push_str(s.next().unwrap());
        b.push(' ');
        b.push_str(s.next().unwrap());

        return Bag::from_str(&b);
    }

    fn colour(&self) -> &str {
        return &self.colour;
    }

    fn contains(&self, rules: &Ruleset, other: &Bag) -> bool {
        // Get direct children of this bag.
        let direct_children = self.contains_directly(rules);

        // Base case: No direct children, return false.
        // Base case: Direct children contains search item, return true.
        // Recursive case: At least one direct child.
        //     Return the union of the sets that each direct child
        //     contains.
        if direct_children.len() == 0 {
            return false;
        } else if direct_children.contains_key(other) {
            return true;
        } else {
            for (child_bag, _) in direct_children {
                if child_bag.contains(rules, other) {
                    return true
                }
            }

            // No child bag contains the search bag, return false.
            return false;
        }
    }

    fn contains_directly<'a>(&self, rules: &'a Ruleset) -> &'a HashMap<Bag, u32> {
        return rules.contains(self);
    }
}

fn part1() -> u32 {
    // Read rules into file.
    let rules = fs::read_to_string("data/day7.txt").unwrap();
    let rules = Ruleset::from_str(&rules);

    // Collect all the types of bags.
    let bags = rules.bags();

    // We're looking for bags that contain "shiny gold".
    let search_bag = Bag::from_str("shiny gold");
    let mut count = 0;

    for bag in bags {
        if bag.contains(&rules, &search_bag) {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let count_part1 = part1();
    println!("Part 1: Number of bags that contain shiny gold is: {}", count_part1);
}
