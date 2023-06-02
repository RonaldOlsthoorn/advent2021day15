use std::io::{BufReader, BufRead};
use std::fs::File;
use std::iter::{zip, repeat};
use petgraph::Directed;
use petgraph::prelude::DiGraph;
use petgraph::{matrix_graph::{MatrixGraph}, algo::astar, visit::EdgeRef};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
struct Point {
    x: u16,
    y: u16
}

fn main() {

    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines().map(|l| l.unwrap()).collect();

    let width = lines[0].len();
    let height = lines.len();

    let total_width = 5 * width;
    let total_height = 5 * height;

    println!("total_width: {} total_height {}, cap {}", total_width, total_height, total_width * total_height);

    let mut graph: DiGraph<Point, u8, u32> = DiGraph::with_capacity(total_width * total_height, total_width * total_height * 4);

    for (row_index, line) in (0..5).flat_map(|_| lines.iter()).enumerate() {
        for (col_index, _) in (0..5).flat_map(|_| line.chars()).enumerate() {

            let node = Point{x: col_index as u16, y: row_index as u16};
            graph.add_node(node);
        }
    }

    println!("added all nodes");

    for (row_index, (line, level_row)) 
    in (0..5).flat_map(|l| zip(lines.iter(), repeat(l))).enumerate() {
        for (col_index, (weight, level_col)) 
        in (0..5).flat_map(|l| 
                zip(line.chars().map(|c| c.to_digit(10).unwrap() as u8), repeat(l))).enumerate() {
            
            let center_node_index: u32 = (row_index * total_width + col_index) as u32;
            let mut risk = weight + level_row + level_col;

            risk -= ((risk - 1) / 9) * 9;
            
            if row_index > 0 {
                
                let up_node_index: u32 = ((row_index - 1) * total_width + col_index) as u32;
                graph.add_edge(
                    up_node_index.into(),
                    center_node_index.into(),
                    risk);
            }

            if row_index < total_height - 1 {

                let down_node_index: u32 = ((row_index + 1) * total_width + col_index) as u32;
                graph.add_edge(
                    down_node_index.into(),
                    center_node_index.into(), 
                    risk);
            }

            if col_index > 0 {

                let left_node_index: u32 = (row_index * total_width + col_index - 1) as u32;
                graph.add_edge(
                    left_node_index.into(),
                    center_node_index.into(),
                    risk);
            }

            if col_index < total_width - 1 {

                let right_node_index: u32 = (row_index * total_width + col_index + 1) as u32;
                graph.add_edge(
                    right_node_index.into(),
                    center_node_index.into(),
                    risk);
            }
        }
    }

    println!("running astar...");


    let goal_point = Point{x: total_width as u16 - 1, y: total_height as u16 -1};

    let path = astar(
        &graph, 
        0.into(),
        |n| graph.node_weight(n).unwrap() == &goal_point,
        |e| *e.weight() as u32,
        |n| {
            let p = graph.node_weight(n).unwrap();
                        
            return (total_width as u32 - 1 - p.x as u32) + (total_height as u32 - 1 - p.y as u32);
        } 
    ).unwrap();

    let mut res = 0;

    for (node_index_from, node_index_to)
        in zip(path.1[0..path.1.len()-1].iter(), path.1[1..].iter()) {
        println!("point {:?}", graph.node_weight(*node_index_from));
        res += *graph.edges_connecting(*node_index_from, *node_index_to).next().unwrap().weight() as u32
    }

    println!("total res {}", res);

    //println!("{:?}", Dot::with_config(&graph, &[Config::EdgeIndexLabel, Config::NodeIndexLabel]));

}
