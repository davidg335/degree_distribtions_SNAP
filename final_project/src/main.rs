mod read_file;
mod degree_distribution;
use std::collections::HashSet;
use std::collections::HashMap;
use read_file::*;
use degree_distribution::*;
type Vertex=usize;
type Distance=usize;
type Degree=usize;

fn main() {
    let (data,vertex_list) = read_file("data.txt");
    //println!("{:?}", data);
    //println!("{:?}", vertex_list);
    let adj_list: HashMap<usize,Vec<Vertex>> = generate_adjacency_list(&data,vertex_list);
   // println!("{:?}", adj_list);
   let DIST_RANGE = (0,6);
   let deg_distribution: HashMap<usize,Vec<Vertex>> = compute_all_degrees(&adj_list,DIST_RANGE);
   //println!("{:?}", deg_distribution);
   let means: Vec<(usize,f64)> = calculate_distribution_means(&deg_distribution);
   println!("{:?}", means);

}

#[test] 
fn test_compute_degrees_at_node() {
    let mut data:HashMap<usize,Vec<Vertex>> = HashMap::new();
    data.insert(1,vec![4,6]);
    data.insert(2, vec![3,4]);
    data.insert(3, vec![2,4,5]);
    data.insert(4, vec![1,2,3,6]);
    data.insert(5, vec![3]);
    data.insert(6, vec![1,4]);
    let start_node = 1;
    let max_distance = 5;
    let correct_return = vec![1,2,2,1];
    let actual_return = compute_degrees_at_node(start_node,max_distance, &data);
    assert_eq!(correct_return.len(), actual_return.len(), "Vector return length is incorrect.");
    for index in 0..correct_return.len() {
        assert_eq!(actual_return[index], correct_return[index], "Vector return elements do not match.");
    }
}

#[test]
fn test_compute_all_degrees(){
    let mut data:HashMap<usize,Vec<Vertex>> = HashMap::new();
    data.insert(1,vec![4,6]);
    data.insert(2, vec![3,4]);
    data.insert(3, vec![2,4,5]);
    data.insert(4, vec![1,2,3,6]);
    data.insert(5, vec![3]);
    data.insert(6, vec![1,4]);
    let dist_range = (1,3); //including 1, excluding 3
    let mut correct_return: HashMap<Distance,Vec<Degree>> = HashMap::new();
    correct_return.insert(1,vec![0,1,3,1,1,0]);
    correct_return.insert(2,vec![0,1,4,1,0,0]);
    let actual_return:HashMap<Distance,Vec<Degree>> = compute_all_degrees(&data,dist_range);
    for index in dist_range.0..dist_range.1 {
        assert_eq!(actual_return.get(&index), correct_return.get(&index), "Vector return elements do not match.");
    }
}

#[test]
fn test_degree_zero_compute_all_degrees(){
    let mut data:HashMap<usize,Vec<Vertex>> = HashMap::new();
    data.insert(1,vec![4,6]);
    data.insert(2, vec![3,4]);
    data.insert(3, vec![2,4,5]);
    data.insert(4, vec![1,2,3,6]);
    data.insert(5, vec![3]);
    data.insert(6, vec![1,4]);
    let dist_range = (0,1); //including 0, excluding 1
    let mut correct_return: HashMap<Distance,Vec<Degree>> = HashMap::new();
    correct_return.insert(0,vec![0,6,0,0,0,0]);
    let actual_return:HashMap<Distance,Vec<Degree>> = compute_all_degrees(&data,dist_range);
    for index in dist_range.0..dist_range.1 {
        assert_eq!(actual_return.get(&index), correct_return.get(&index), "Vector return elements do not match.");
    }
}

#[test]
fn test_mean(){
    let data: Vec<Degree> = vec![10,32,4,31,6];
    let correct_return = ((10*0+32*1+4*2+31*3+6*4) as f64)/(5 as f64);
    let actual_return = mean(&data);
    assert_eq!(actual_return, correct_return, "Mean return elements do not match.");
}

#[test]
fn test_calculate_distribution_means(){
    let mut data:HashMap<Distance,Vec<Degree>> = HashMap::new();
    data.insert(2,vec![10,32,4,31,5]);
    data.insert(3,vec![1,2,4,0,5]);
    let mut correct_return: Vec<(usize,f64)> = Vec::new();
    correct_return.push((2,((10*0+32*1+4*2+31*3+5*4) as f64)/5.0));
    correct_return.push((3,((1*0+2*1+4*2+0*3+5*4)as f64)/5.0));
    let actual_return: Vec<(usize,f64)> = calculate_distribution_means(&data);
    for index in 0..2{
        assert_eq!(actual_return[index], correct_return[index], "Mean return elements do not match.");
    }
    
}