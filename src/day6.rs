use crate::solver::Solver;
use petgraph::prelude::*;
use petgraph::Undirected;

pub struct Day6 {}

impl<'a> Solver<'a> for Day6 {
    type Generated = GraphMap<&'a str, u32, Undirected>;
    type Output = u32;

    fn generator(input: &'a str) -> Self::Generated {
        let mut graph = GraphMap::new();

        for l in input.lines() {
            let mut parts = l.split(')');
            let parent = parts.next().unwrap();
            let child = parts.next().unwrap();
            graph.add_edge(parent, child, 1);
        }

        graph
    }

    fn part1(graph: Self::Generated) -> Self::Output {
        let root = "COM";
        petgraph::algo::dijkstra(&graph, root, None, |e| *e.weight())
            .values()
            .sum()
    }

    fn part2(graph: Self::Generated) -> Self::Output {
        let destination = "SAN";
        petgraph::algo::dijkstra(&graph, "YOU", Some(destination), |e| *e.weight())[&destination]
            - 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6p1() {
        assert_eq!(
            Day6::part1(Day6::generator(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
            )),
            42
        );
    }

    #[test]
    fn d6p2() {
        assert_eq!(
            Day6::part2(Day6::generator(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
            )),
            4
        );
    }
}
