use std::{collections::HashMap, env, fs::read_to_string};
fn main() {
    let file = read_to_string("input.txt").expect("these should be an input.txt file next to the executable, with read access.");
    let mut conversions = HashMap::new();
    read_file(file, &mut conversions);

    //user input
    let input: Vec<String> = env::args().collect();
    let (start_unit,goal_unit,value) = match parse_input(&input) {
        Some((start_unit,goal_unit,value)) => (start_unit,goal_unit,value),
        None => {
            println!("usage:\n\tnumber unit to unit\n\tnumber unit -> unit\nexample:\n\t500 mm to yd\n\t91.44 cm to yd");
            return;
        }
    };

    //now let's give a call to the main logic function
    let mut blacklist = Vec::new();
    if let Some(converted) = the_brain(&conversions, start_unit.to_string(), goal_unit, value, &mut blacklist){
        println!("{value} {start_unit} = {converted} {goal_unit}")
    }else{
        println!("\"{start_unit}\" or \"{goal_unit}\" does not exist in conversion TREE!");
    };

}
// when going down the tree blacklist every element that you have already visited, this way it's not even needed to keep track of what path you went down or not, but check if the next node exists in the blacklist vector, if it does check other branch, if all branches are blacklisted return none, if the branch is not blacklisted, recall the fucntion with he new index, and recalculated values. Start the fucntion with blacklisting the node that you are on. While writing this I realised that you do still need to keep track of branches to iterate thru them, but it's regular ass for loop so. fn(tree_vector,current_node_index,value,visited_nodes_hashmap) struct tree(unit_name,[[times amount,index]_conversion]_vector)
// if tree_vector[tree.1[1]] in visited_nodes_hashmap: true means it's blacklisted, if not please do visit

fn the_brain(tree: &HashMap<String, Vec<(String,f32)>>, unit: String, goal_unit: &str, value: f32, blacklist: &mut Vec<String>) -> Option<f32>{
    if unit == goal_unit {
        return Some(value);
    }
    blacklist.push(unit.to_owned());
    if let Some(node) = tree.get(&unit) {
        for conversion in node {
            if blacklist.contains(&conversion.0) {continue;}//anti-loop
            let ret = the_brain(tree, conversion.0.to_string(), goal_unit, value*conversion.1, blacklist);
            if ret.is_none(){
                continue;
            }else{
                return ret;
            }
        }
        return None;
    }
    None//this will only happen if the user input is wrong
}

fn read_file(file: String, tree: &mut HashMap<String, Vec<(String,f32)>>) {
    for line in file.lines() {
        // float unit = float unit
        let parts: Vec<(f32, &str)> = 
            line
            .split("=")
            .map(|f|{
                let f: Vec<&str> = f.split_whitespace().collect();
                (f[0].parse::<f32>().unwrap(),f[1])
            })
            .collect::<Vec<(f32,&str)>>();
        //we need a unit to unit and transfer rate which it x/1*2=2x/1=x*(2/1)
        //to turn unit_1 into unit_2 we need to multiply it by float_2/float_1
        let conversion_from_to = parts[1].0/parts[0].0;
        let conversion_to_from = parts[0].0/parts[1].0;
        let from = parts[0].1.to_string();
        let to = parts[1].1.to_string();
        let entry = tree.entry(from.to_string()).or_insert(Vec::new());
        entry.push((to.to_string(),conversion_from_to));
        let entry: &mut Vec<(String, f32)> = tree.entry(to).or_insert(Vec::new());
        entry.push((from,conversion_to_from));
    }
}

fn parse_input<'a>(input:&'a Vec<String>) -> Option<(&'a str,&'a str,f32)> {
    if input.get(3)? != "to" && input.get(2)? != "->" {
        return None;
    }
    return Some((
        input.get(2)?,
        input.get(4)?,
        input.get(1)?.parse().ok()?,
    ));
}