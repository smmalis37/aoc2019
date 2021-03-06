use crate::solver::Solver;
use std::collections::{HashMap, VecDeque};

pub struct Day14 {}

type Num = u64;

#[derive(Clone, Copy)]
struct RecipePart<'a> {
    chemical: &'a str,
    amount: Num,
}

impl<'a> RecipePart<'a> {
    fn new(s: &'a str) -> Self {
        let space = s.find(' ').unwrap();
        Self {
            chemical: &s[space + 1..],
            amount: s[..space].parse().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct Recipe<'a> {
    inputs: Vec<RecipePart<'a>>,
    output: RecipePart<'a>,
}

impl<'a> Solver<'a> for Day14 {
    type Generated = HashMap<&'a str, Recipe<'a>>;
    type Output = Num;

    fn generator(input: &'a str) -> Self::Generated {
        input
            .lines()
            .map(|l| {
                let arrow = l.find("=>").unwrap();
                let inputs = l[..arrow]
                    .trim()
                    .split(", ")
                    .map(|x| RecipePart::new(x))
                    .collect();
                let output = RecipePart::new(l[arrow + 2..].trim());
                (output.chemical, Recipe { inputs, output })
            })
            .collect()
    }

    fn part1(recipes: Self::Generated) -> Self::Output {
        calculate_ore_count(&recipes, 1)
    }

    fn part2(recipes: Self::Generated) -> Self::Output {
        const GOAL_ORE: Num = 1_000_000_000_000;

        let mut fuel_count = 1;
        let mut ore_used = calculate_ore_count(&recipes, fuel_count);

        while ore_used < GOAL_ORE {
            fuel_count = ((GOAL_ORE as f64 / ore_used as f64) * fuel_count as f64) as Num + 1;
            ore_used = calculate_ore_count(&recipes, fuel_count);
        }

        while ore_used > GOAL_ORE {
            fuel_count -= 1;
            ore_used = calculate_ore_count(&recipes, fuel_count);
        }

        fuel_count
    }
}

fn calculate_ore_count(recipes: &<Day14 as Solver>::Generated, fuel_count: Num) -> Num {
    let mut queue = VecDeque::new();
    let mut ore_count = 0;
    let mut leftovers = HashMap::<&str, Num>::new();
    queue.push_back(RecipePart {
        amount: fuel_count,
        chemical: "FUEL",
    });

    while let Some(mut wanted) = queue.pop_back() {
        if wanted.chemical == "ORE" {
            ore_count += wanted.amount;
        } else {
            let recipe = &recipes[wanted.chemical];

            let previous_leftover = *leftovers.get(wanted.chemical).unwrap_or(&0);
            leftovers.insert(
                wanted.chemical,
                previous_leftover.saturating_sub(wanted.amount),
            );
            wanted.amount = wanted.amount.saturating_sub(previous_leftover);

            let recipe_times = wanted.amount / recipe.output.amount
                + if wanted.amount % recipe.output.amount != 0 {
                    1
                } else {
                    0
                };
            let produced_amount = recipe.output.amount * recipe_times;
            let new_leftovers = produced_amount - wanted.amount;
            *leftovers.entry(wanted.chemical).or_insert(0) += new_leftovers;

            for i in &recipe.inputs {
                let mut c = *i;
                c.amount *= recipe_times;
                queue.push_back(c);
            }
        }
    }

    ore_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d14p1() {
        assert_eq!(
            Day14::part1(Day14::generator(
                "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
            )),
            165
        );

        assert_eq!(
            Day14::part1(Day14::generator(
                "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            )),
            13312
        );

        assert_eq!(
            Day14::part1(Day14::generator(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"
            )),
            180_697
        );

        assert_eq!(
            Day14::part1(Day14::generator(
                "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"
            )),
            2_210_736
        );
    }

    #[test]
    fn d14p2() {
        assert_eq!(
            Day14::part2(Day14::generator(
                "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            )),
            82_892_753
        );

        assert_eq!(
            Day14::part2(Day14::generator(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"
            )),
            5_586_022
        );

        assert_eq!(
            Day14::part2(Day14::generator(
                "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"
            )),
            460_664
        );
    }
}
