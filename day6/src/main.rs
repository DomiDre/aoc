use std::fs::File;
use std::io::{Error, BufRead, BufReader};

/// Save data as tree, define a leaf with storing the data
/// and pointers to its childrens
#[derive(Debug)]
struct OrbitLeaf {
    name: String,
    children: Vec<OrbitLeaf>,
}

impl OrbitLeaf {
    fn new(name: &str) -> OrbitLeaf {
        OrbitLeaf {
            name: String::from(name),
            children: Vec::new()
        }
    }

    fn add_leaf(self: &mut Self, new_leaf: OrbitLeaf) {
        self.children.push(new_leaf);
    }

    fn get_child_by_name(&mut self, name: String) -> Option<&mut OrbitLeaf> {
        if self.name == name {
            return Some(self)
        }
        for child in self.children.iter_mut() {
            let search_child = child.get_child_by_name(name.clone());
            if search_child.is_some() {
                return search_child
            }
        }
        None
    }

    fn is_child_in_tree(&self, name: String) -> bool {
        // is node itself the one searched for?
        if self.name == name {
            return true
        }
        // any of the children the name searched for?
        for child in self.children.iter() {
            let search_child = child.is_child_in_tree(name.clone());
            if search_child {
                return search_child
            }
        }
        // so the answer is no for this path
        false
    }

    fn get_idx(&self, name: String) -> Option<usize> {
        for (i, child) in self.children.iter().enumerate() {
            if child.name == name {
                return Some(i)
            }
        }
        None
    }

    fn get_parent_by_child_name(&self, name: String) -> Option<&OrbitLeaf> {
        for child in self.children.iter() {
            if child.name == name {
                return Some(self)
            }
        }
        for child in self.children.iter() {
            let search_child = child.get_parent_by_child_name(name.clone());
            if search_child.is_some() {
                return search_child
            }
        }
        None
    }

    fn calc_checksum(&self, level: usize) -> usize {
        let mut child_sum = 0;
        for child in self.children.iter() {
            child_sum += child.calc_checksum(level+1);
        }
        child_sum + level

    }

}

fn construct_tree() -> Result<OrbitLeaf, Error> {
    let mut orbit_tree = OrbitLeaf::new("COM");

    let file = File::open("./input")?;
    let buffered = BufReader::new(file);
    'line_loop: for line in buffered.lines() {
        let input = line.unwrap();
        let split: Vec<_> = input.split(")").collect();
        let center = split[0];
        let orbiter = split[1];
        
        // check if center and orbiter are already in the tree
        let is_center_in_tree = orbit_tree.is_child_in_tree(
            String::from(center)
        );  
        let is_orbiter_in_tree = orbit_tree.is_child_in_tree(
            String::from(orbiter)
        );  
        
        // center is already in tree
        if is_center_in_tree && !is_orbiter_in_tree {
            // if orbit is not already in tree -> just add new leaf
            let center_node = orbit_tree.get_child_by_name(
                String::from(center)
            ).unwrap(); 
            center_node.add_leaf(
                OrbitLeaf::new(orbiter)
            );
        } else if is_center_in_tree && is_orbiter_in_tree {
            // center is in tree & orbiter is in tree
            // if orbiter is not on the top level this case would be ambigous
            // so just check top level
            let idx = orbit_tree.get_idx(String::from(orbiter));
            if idx.is_none() {
                panic!("More tricky case in data?");
            }
            // remove orbiter from root, and add to center_node
            let orbiter_node = orbit_tree.children.remove(idx.unwrap());
            let center_node = orbit_tree.get_child_by_name(
                String::from(center)
            ).unwrap(); 
            center_node.add_leaf(orbiter_node);
        } else if !is_center_in_tree && is_orbiter_in_tree {
            // center is not in tree yet, but the orbiter
            // orbiter should be top level otherwise tricky
            let idx = orbit_tree.get_idx(String::from(orbiter));
            if idx.is_none() {
                panic!("Orbiter in tree, center not, but orbiter not on top level?");
            }
            let orbiter_node = orbit_tree.children.remove(idx.unwrap());
            let mut center_node = OrbitLeaf::new(center);
            center_node.add_leaf(orbiter_node);
            orbit_tree.add_leaf(center_node);
        } else {
            // nothing is yet in tree, just add
            let mut new_leaf = OrbitLeaf::new(center);
            new_leaf.add_leaf(OrbitLeaf::new(orbiter));
            orbit_tree.add_leaf(new_leaf);
        }
    }
    Ok(orbit_tree)
}

/// get checksum for tree
fn part1() {
    let orbit_tree = construct_tree().unwrap();
    println!("{}", orbit_tree.calc_checksum(0));
}


/// find number of orbital steps between SAN and YOU
/// inefficient implementation but doesnt matter
fn part2() {
    let orbit_tree = construct_tree().unwrap();
    
    let mut SAN_parent = orbit_tree.get_parent_by_child_name(
        String::from("SAN")
    ).unwrap();
    
    let mut orbital_jumps = 0;
    // find mutual parent of SAN and YOU
    'san_loop: loop {
        let mut YOU_parent = orbit_tree.get_parent_by_child_name(
            String::from("YOU")
        ).unwrap(); 
        let mut you_jumps = 0;
        'you_loop: loop {
            if YOU_parent.name == SAN_parent.name {
                break;
            }
            YOU_parent = orbit_tree.get_parent_by_child_name(
                YOU_parent.name.clone()
            ).unwrap();
            you_jumps += 1;
            if YOU_parent.name == String::from("COM") {
                break;
            }
        }
        if YOU_parent.name == SAN_parent.name {
            orbital_jumps += you_jumps;
            break;
        }
        SAN_parent = orbit_tree.get_parent_by_child_name(
            SAN_parent.name.clone()
        ).unwrap();
        orbital_jumps += 1;
    }
    println!("{:?}", orbital_jumps);
}


fn main() {
    // part1();
    part2();
}
