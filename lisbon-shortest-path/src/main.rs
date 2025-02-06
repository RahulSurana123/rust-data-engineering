use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::visit::NodeRef;
use std::io::stdin;

fn read() -> usize {
    let mut val = String::new();
    stdin().read_line(&mut val);
    val.trim().parse().unwrap()
}

fn main() {
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
        (lx_factory, commerce_square, 10), // new edge from LX Factory to Commerce Square is 10 km
    ]);

    for (i, node) in graph.node_weights().into_iter().enumerate() {
        println!(" {i}. {node} ",);
    }
    
    println!("Please Select source Node number : ");
    let src = read();
    println!("Please Select destination Node number : ");
    let dest = read();

    let src_name = graph.node_weights().into_iter().nth(src);
    let dest_name = graph.node_weights().into_iter().nth(dest);
    let src = graph.node_indices().into_iter().nth(src).unwrap();
    let dest = graph.node_indices().into_iter().nth(dest);

    let node_map = dijkstra(&graph, src, dest, |e| *e.weight());

    if let Some(distance) = node_map.get(&dest.unwrap()) {
        println!(
            "The shortest distance from {:?} to {:?} is {} km",
            src_name.unwrap(),
            dest_name.unwrap(),
            distance
        );
    } else {
        println!(
            "No route found from {:?} to {:?}.",
            src_name.unwrap(),
            dest_name.unwrap()
        );
    }
}
