type Vertex = usize;
type Distance = usize;
type Degree = usize;
use std::collections::HashMap;
use std::collections::VecDeque;


pub fn compute_degrees_at_node(start_node: Vertex, absolute_max_distance: usize, data: &HashMap<usize,Vec<Vertex>>)->Vec<Degree>{
    let mut distance: HashMap<Vertex,Option<usize>> = HashMap::new();//distances each node is away from start node
    for (node,_edges) in data{
        distance.insert(*node,None);
    }
    let mut included_nodes: Vec<Vertex>= Vec::new();
    included_nodes.push(start_node);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    let mut actual_max_distance: usize = 0;
    distance.insert(start_node, Some(0));
    queue.push_back(start_node);
    while let Some(v) = queue.pop_front() { 
       println!("top {:?}",queue);
        for u in data.get(&v).unwrap().iter() {
            if None == distance.get(u).unwrap().as_ref() { 
                if distance.get(&v).unwrap().unwrap()>absolute_max_distance{
                    break;
                }
                distance.insert(*u,Some(distance.get(&v).unwrap().unwrap() + 1));
                included_nodes.push(*u);
                actual_max_distance = distance.get(u).unwrap().unwrap();
                queue.push_back(*u);
                println!("In {:?}",queue);
            }
        }
    };
    println!("{:?}",distance);
    let mut degrees: Vec<Degree> = vec![0;actual_max_distance+1]; //index is the distance, values are the degrees at that distance
    //println!("Test A, {:?}", actual_max_distance);
    for node in included_nodes{
       // println!("Test B");
        degrees[distance.get(&node).unwrap().unwrap()]+=1;
        
    }
   // println!("Test C");
    return degrees; // returns the degree value for each distance up to its max distance
}

pub fn compute_all_degrees(data: &HashMap<usize,Vec<Vertex>>, dist_range: (usize,usize))->HashMap<Distance,Vec<Degree>> {
    let mut degree_dist_by_distance: HashMap<Distance,Vec<Degree>> = HashMap::new();
    for distance in dist_range.0..dist_range.1{
        degree_dist_by_distance.insert(distance,vec![0;data.len()]);
    }
    for (node,_edges) in data{
        let degrees: Vec<Degree> = compute_degrees_at_node(*node,dist_range.1-1,data); //degrees indexed by distance for node "node"
        for distance in dist_range.0..dist_range.1{ // for each distance of interest, add degree of node for that distance to "degree_dist_by_distance"
            if degrees.len()>distance{
                degree_dist_by_distance.get_mut(&distance).unwrap()[degrees[distance]]+=1;
                // degree_dist_by_distance.insert(distance,degree_dist_by_distance.get(&distance).unwrap()[degrees[distance]]+1);
            }
        }
    }
    //println!("{:?}", degree_dist_by_distance);
    return degree_dist_by_distance;
}

pub fn mean(degree_dist: &Vec<Degree>)->f64{
    let mut mean: f64 = 0.0;
    for index in 0..degree_dist.len(){
        mean+=(index*degree_dist[index])as f64;
    }
    return mean/(degree_dist.len() as f64);
}

pub fn calculate_distribution_means(distribution_data: &HashMap<Distance,Vec<Degree>>)-> Vec<(usize,f64)>{
    let mut sorted_distances: Vec<usize> = Vec::new();
    for (dist,_deg) in distribution_data{
        sorted_distances.push(*dist);
    }
    sorted_distances.sort();
    let mut means: Vec<(usize,f64)> = Vec::new();
    for dist in sorted_distances{
        means.push((dist,mean(&distribution_data.get(&dist).unwrap())));
    }
    return means;
}

