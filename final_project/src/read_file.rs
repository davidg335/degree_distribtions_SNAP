use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;


type Vertex = usize;

pub fn read_file(path_name: &str) -> (Vec<(Vertex, Vertex)>,HashSet<Vertex>) {
    let mut result: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open(path_name).expect("Cannot open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut lineno = 0;
    let mut vertex_hashset = HashSet::new();

    for line in buf_reader {
        lineno += 1;
        let s:String = format!("Line no {}", lineno);
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        if v.len() == 2 {
            let name = v[0].parse::<usize>().expect(&s);
            let edge = v[1].parse::<usize>().expect(&s);
            result.push((name, edge));
            vertex_hashset.insert(name);
        }
    }
    return (result,vertex_hashset);
}

pub fn generate_adjacency_list(data: &Vec<(Vertex,Vertex)>, vertex_hashset: HashSet<Vertex>) -> HashMap<usize,Vec<Vertex>>{
    let mut adj_list = HashMap::new();
    for key in vertex_hashset.iter() {
        adj_list.insert(*key,Vec::new());
    };
    
    for (node,edge) in data{
        adj_list.get_mut(node).map(|n| n.push(*edge));
      //  adj_list.get_mut(edge).map(|n| n.push(*node));  // this is already built into the dataset
    }
    return adj_list;
}









