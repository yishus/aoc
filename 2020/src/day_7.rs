use std::collections::HashMap;
use std::fs;
use regex::Regex;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};
use petgraph::algo::has_path_connecting;

pub fn a() {
    let contents = fs::read_to_string("input/day_7.txt")
        .expect("Something went wrong reading the file");
    
    let source_re = Regex::new(r"([a-z ]+) bags contain").unwrap();
    let target_re = Regex::new(r"[1-9] ([a-z ]+) bag").unwrap();

    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    let mut g = Graph::<&str, ()>::new();

    let rules: Vec<&str> = contents
        .split('\n')
        .collect();

    for r in rules {
        let caps = source_re.captures(r).unwrap();
        let source = caps.get(1).map_or("", |m| m.as_str());
        let source_node = match nodes.get(source) {
            Some(ni) => *ni,
            None => {
                let node = g.add_node(source);
                nodes.insert(source, node);
                node
            }
        };
        
        for caps in target_re.captures_iter(r) {
            let target = caps.get(1).map_or("", |m| m.as_str());
            let target_node = match nodes.get(target) {
                Some(ni) => *ni,
                None => {
                    let node = g.add_node(target);
                    nodes.insert(target, node);
                    node
                }
            };
            g.add_edge(source_node, target_node, ());
        }
    }

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