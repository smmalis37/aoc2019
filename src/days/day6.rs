use crate::solver::Solver;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Undirected;

pub struct Day6 {}

type N = u32;

impl<'a> Solver<'a> for Day6 {
    type Generated = GraphMap<&'a str, N, Undirected>;
    type Output = N;

    fn generator(input: &'a str) -> Self::Generated {
        let mut graph = GraphMap::new();

        for l in input.lines() {
            let separator = l.find(')').unwrap();
            let parent = &l[0..separator];
            let child = &l[separator + 1..];
            graph.add_edge(parent, child, 1);
        }

        graph
    }

    fn part1(graph: Self::Generated) -> Self::Output {
        let root = "COM";
        dijkstra(&graph, root, None, |e| *e.weight()).values().sum()
    }

    fn part2(graph: Self::Generated) -> Self::Output {
        let destination = "SAN";
        dijkstra(&graph, "YOU", Some(destination), |e| *e.weight())[&destination] - 2
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
