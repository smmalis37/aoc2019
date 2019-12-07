use petgraph::prelude::*;
use petgraph::Undirected;

pub fn generator(input: &str) -> GraphMap<&str, u32, Undirected> {
    let mut graph = GraphMap::new();

    for l in input.lines() {
        let mut parts = l.split(')');
        let parent = parts.next().unwrap();
        let child = parts.next().unwrap();
        graph.add_edge(parent, child, 1);
    }

    graph
}

pub fn part1(graph: GraphMap<&str, u32, Undirected>) -> u32 {
    let root = "COM";
    petgraph::algo::dijkstra(&graph, root, None, |e| *e.weight())
        .values()
        .sum()
}

pub fn part2(graph: GraphMap<&str, u32, Undirected>) -> u32 {
    let destination = "SAN";
    petgraph::algo::dijkstra(&graph, "YOU", Some(destination), |e| *e.weight())[&destination] - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6p1() {
        assert_eq!(
            part1(generator(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"
            )),
            42
        );
    }

    #[test]
    fn d6p2() {
        assert_eq!(
            part2(generator(
                "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
            )),
            4
        );
    }
}
