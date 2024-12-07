use std::collections::{HashMap, HashSet};

use crate::utils::read_lines_as_int_arrays::read_lines_as_int_arrays;

pub fn run_a() -> std::io::Result<()> {
    let page_ordering_rules = read_lines_as_int_arrays("src/day_5/page_ordering_rules.txt", "|")?;
    let pages_to_produce = read_lines_as_int_arrays("src/day_5/pages_to_produce.txt", ",")?;
    let (correctly_ordered_updates, _) = get_correct_and_incorrect_orderings(page_ordering_rules.clone(), pages_to_produce);

    // get sum of middle page numbers for correctly ordered updates
    let mut sum = 0;
    for pages in correctly_ordered_updates {
        let middle_page_index = pages.len() / 2;
        sum += pages[middle_page_index];
    }

    println!("Sum of middle page numbers for correctly ordered updates: {}", sum);
    
    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let page_ordering_rules = read_lines_as_int_arrays("src/day_5/page_ordering_rules.txt", "|")?;
    let pages_to_produce = read_lines_as_int_arrays("src/day_5/pages_to_produce.txt", ",")?;

    let (_, incorrectly_ordered_updates) = get_correct_and_incorrect_orderings(page_ordering_rules.clone(), pages_to_produce);
    
    let mut sum = 0;
    for pages in incorrectly_ordered_updates {
        let fixed_order = topological_sort(&pages, &page_ordering_rules);
        let middle_index = fixed_order.len() / 2;
        sum += fixed_order[middle_index];
    }

    println!("Sum of middle page numbers for fixed ordered updates: {}", sum);
    
    Ok(())
}

fn get_correct_and_incorrect_orderings(page_ordering_rules: Vec<Vec<i32>>, pages_to_produce: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut correctly_ordered_updates: Vec<Vec<i32>> = Vec::new();
    let mut incorrectly_ordered_updates: Vec<Vec<i32>> = Vec::new();

    for pages in pages_to_produce {
        let mut correctly_ordered_update = true;

        for i in 0..pages.len() {
            let current_page = pages[i];
            let pages_before_current = &pages[0..i];
            let pages_after_current = &pages[i+1..];

            // pages before current must be present on the left hand side of the page ordering rules
            let left_hand_sides = find_left_hand_sides_of_page_ordering_rules_given_page_number(current_page, page_ordering_rules.clone());
            for page in pages_before_current {
                if !left_hand_sides.contains(page) {
                    correctly_ordered_update = false;
                    break;
                }
            }

            // pages after current must be present on the right hand side of the page ordering rules
            let right_hand_sides = find_right_hand_sides_of_page_ordering_rules_given_page_number(current_page, page_ordering_rules.clone());
            for page in pages_after_current {
                if !right_hand_sides.contains(page) {
                    correctly_ordered_update = false;
                    break;
                }
            }
        }

        if correctly_ordered_update {
            correctly_ordered_updates.push(pages);
        } else {
            incorrectly_ordered_updates.push(pages);
        }
    }

    return (correctly_ordered_updates, incorrectly_ordered_updates);
}

fn find_left_hand_sides_of_page_ordering_rules_given_page_number(page_number: i32, page_ordering_rules: Vec<Vec<i32>>) -> Vec<i32> {
    let mut left_hand_sides: Vec<i32> = Vec::new();
    for rule in page_ordering_rules {
        if rule[1] == page_number {
            left_hand_sides.push(rule[0]);
        }
    }
    return left_hand_sides;
}

fn find_right_hand_sides_of_page_ordering_rules_given_page_number(page_number: i32, page_ordering_rules: Vec<Vec<i32>>) -> Vec<i32> {
    let mut right_hand_sides: Vec<i32> = Vec::new();
    for rule in page_ordering_rules {
        if rule[0] == page_number {
            right_hand_sides.push(rule[1]);
        }
    }
    return right_hand_sides;
}

fn topological_sort(pages: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<i32> {
    // Build adjacency list and in-degree count for pages
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, i32> = HashMap::new();
    let page_set: HashSet<i32> = pages.iter().cloned().collect();

    // Initialize in-degree for all pages
    for &page in pages {
        graph.entry(page).or_insert(Vec::new());
        in_degree.entry(page).or_insert(0);
    }

    // Build the graph using only rules that involve pages in our set
    for rule in rules {
        let from = rule[0];
        let to = rule[1];
        if page_set.contains(&from) && page_set.contains(&to) {
            graph.entry(from).or_default().push(to);
            *in_degree.entry(to).or_default() += 1;
        }
    }

    // Find all nodes with in-degree 0
    let mut result = Vec::new();
    let mut queue: Vec<i32> = pages.iter()
        .filter(|&&page| in_degree.get(&page).unwrap_or(&0) == &0)
        .cloned()
        .collect();

    while !queue.is_empty() {
        // Sort queue to ensure deterministic ordering when multiple nodes have in-degree 0
        queue.sort_unstable();
        let current = queue.remove(0);
        result.push(current);

        if let Some(neighbors) = graph.get(&current) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push(next);
                }
            }
        }
    }

    result
}