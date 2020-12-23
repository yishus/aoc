use std::collections::HashMap;
use std::fs;
use regex::Regex;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::has_path_connecting;
use petgraph::Direction;

fn bag_graph() -> (Graph<String, usize>, HashMap<String, NodeIndex>) {
    let contents = fs::read_to_string("input/day_7.txt")
        .expect("Something went wrong reading the file");
    
    let source_re = Regex::new(r"([a-z ]+) bags contain").unwrap();
    let target_re = Regex::new(r"([1-9]) ([a-z ]+) bag").unwrap();

    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    let mut g = Graph::<String, usize>::new();

    let rules: Vec<&str> = contents
        .split('\n')
        .collect();

    for r in rules {
        let caps = source_re.captures(r).unwrap();
        let source = caps.get(1).map_or("", |m| m.as_str());
        let source_node = match nodes.get(source) {
            Some(ni) => *ni,
            None => {
                let node = g.add_node(source.to_string());
                nodes.insert(source.to_string(), node);
                node
            }
        };
        
        for caps in target_re.captures_iter(r) {
            let target = caps.get(2).map_or("", |m| m.as_str());
            let count: usize = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
            let target_node = match nodes.get(target) {
                Some(ni) => *ni,
                None => {
                    let node = g.add_node(target.to_string());
                    nodes.insert(target.to_string(), node);
                    node
                }
            };
            g.add_edge(source_node, target_node, count);
        }
    }

    return (g, nodes);
}

pub fn a() {
    let (g, nodes) = bag_graph();

    let mut count = 0;
    let target = nodes["shiny gold"];
    for key in nodes.keys() {
        if key != &"shiny gold" {
            let node = nodes[key];
            let has_path = has_path_connecting(&g, node, target, None);
            if has_path {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

pub fn b() {
    let (g, nodes) = bag_graph();

    let target = nodes["shiny gold"];
    let count = count_bags(&g, target);

    println!("{}", count);
}

fn count_bags(g: &Graph<String, usize>, node: NodeIndex) -> usize {
    let mut count = 0;
    for neighbor in g.neighbors_directed(node, Direction::Outgoing) {
        let edge = g.find_edge(node, neighbor).unwrap();
        let weight = g.edge_weight(edge).unwrap();
        count += weight;
        count += weight * count_bags(g, neighbor);
    }

    return count;
}