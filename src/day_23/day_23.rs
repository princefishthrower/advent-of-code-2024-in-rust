use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Define our network structure
struct Network {
    // Each computer maps to a set of computers it's connected to
    connections: HashMap<String, HashSet<String>>,
}

impl Network {
    fn new() -> Self {
        Network {
            connections: HashMap::new(),
        }
    }

    // Add a bidirectional connection between two computers
    fn add_connection(&mut self, comp1: &str, comp2: &str) {
        self.connections
            .entry(comp1.to_string())
            .or_insert_with(HashSet::new)
            .insert(comp2.to_string());
        
        self.connections
            .entry(comp2.to_string())
            .or_insert_with(HashSet::new)
            .insert(comp1.to_string());
    }

    // Find all sets of n interconnected computers
    fn find_sets_of_n_computers(&self, n: usize) -> HashSet<Vec<String>> {
        let mut result = HashSet::new();
        
        // Get all computer names as a vector
        let computers: Vec<String> = self.connections.keys().cloned().collect();
        
        // Helper function to check if a set of computers are all interconnected
        fn are_all_connected(network: &Network, computers: &[String]) -> bool {
            for i in 0..computers.len() {
                for j in (i + 1)..computers.len() {
                    if !network.connections[&computers[i]].contains(&computers[j]) {
                        return false;
                    }
                }
            }
            true
        }
        
        // Generate combinations of n computers
        fn generate_combinations(
            current: &mut Vec<String>,
            remaining: &[String],
            n: usize,
            network: &Network,
            result: &mut HashSet<Vec<String>>,
        ) {
            if current.len() == n {
                if are_all_connected(network, current) {
                    let mut sorted = current.clone();
                    sorted.sort();
                    result.insert(sorted);
                }
                return;
            }
            
            for i in 0..remaining.len() {
                current.push(remaining[i].clone());
                generate_combinations(
                    current,
                    &remaining[i + 1..],
                    n,
                    network,
                    result,
                );
                current.pop();
            }
        }
        
        let mut current = Vec::new();
        generate_combinations(&mut current, &computers, n, self, &mut result);
        
        result
    }

    // Find the largest set of fully connected computers
    fn find_largest_lan_party(&self) -> Vec<String> {
        let start_time = std::time::Instant::now();
        let mut max_set = Vec::new();
        let computers: Vec<String> = self.connections.keys().cloned().collect();
        println!("Total computers to check: {}", computers.len());

        // Helper function to check if a computer can be added to current set
        fn can_add_to_set(network: &Network, current_set: &[String], computer: &str) -> bool {
            current_set.iter().all(|existing| network.connections[existing].contains(computer))
        }

        // Recursive function to build sets
        fn build_set(
            network: &Network,
            current: &mut Vec<String>,
            remaining: &[String],
            max_set: &mut Vec<String>,
            depth: usize,
        ) {
            // Update max_set if we found a larger valid set
            if current.len() > max_set.len() {
                *max_set = current.clone();
            }

            // Try each remaining computer
            for i in 0..remaining.len() {
                let computer = &remaining[i];
                
                // Check if this computer can connect to all existing ones
                if can_add_to_set(network, current, computer) {
                    current.push(computer.clone());
                    build_set(
                        network,
                        current,
                        &remaining[i + 1..],
                        max_set,
                        depth + 1,
                    );
                    current.pop();
                }
            }
        }

        let mut current = Vec::new();
        build_set(self, &mut current, &computers, &mut max_set, 0);

        let duration = start_time.elapsed();
        println!("Time taken to find largest set: {:?}", duration);

        // Sort alphabetically before returning
        max_set.sort();
        max_set
    }
}

// Parse the input file
fn parse_network<P: AsRef<Path>>(path: P) -> io::Result<Network> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut network = Network::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.trim().split('-').collect();
        if parts.len() == 2 {
            network.add_connection(parts[0], parts[1]);
        }
    }

    Ok(network)
}

pub fn run_a() -> std::io::Result<()> {
    let network = parse_network("./src/day_23/input.txt")?;

    // find all sets of 3 interconnected computers
    let sets_of_3 = network.find_sets_of_n_computers(3);

    // print the total number of sets
    println!("Total number of sets of 3: {}", sets_of_3.len());

    // filter for sets containing at least one computer starting with 't'
    let result = sets_of_3.iter().filter(|set| set.iter().any(|comp| comp.starts_with('t')));

    // print the total number of sets
    println!("Total number of sets of 3 with 't' in first letter of at least one computer: {}", result.count());
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let network = parse_network("./src/day_23/input.txt")?;

    // find the largest set of fully connected computers
    let largest_set = network.find_largest_lan_party();

    // sort the set alphabetically
    let password = largest_set.join(",");
    println!("Password: {}", password);
    
    Ok(())
}
