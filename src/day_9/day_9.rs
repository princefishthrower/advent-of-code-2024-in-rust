use crate::utils::read_as_string::read_as_string;

#[derive(Clone)]
struct FileBlock {
    id: i64,
    location: i64,
}

pub fn run_a() -> std::io::Result<()> {
    let disk_state = read_as_string("src/day_9/input.txt")?;
    
    let file_blocks = build_file_blocks(&disk_state);

    let defragmented_file_blocks = defragment_disk(file_blocks);

    let checksum = calculate_file_checksum(defragmented_file_blocks);

    println!("Checksum: {}", checksum);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let disk_state = read_as_string("src/day_9/input.txt")?;
    
    let file_blocks = build_file_blocks(&disk_state);

    let defragmented_file_blocks = defragment_disk_full_file(file_blocks);

    let checksum = calculate_file_checksum(defragmented_file_blocks);

    println!("Checksum: {}", checksum);

    Ok(())
}

fn build_file_blocks(disk_state: &str) -> Vec<FileBlock> {
    let mut file_blocks = Vec::<FileBlock>::new();
    let mut current_file_id = 0i64;
    let mut current_position = 0i64;
    
    for (i, c) in disk_state.chars().enumerate() {
        if i % 2 == 0 {
            let file_size = c.to_string().parse::<i64>().unwrap();
            for _ in 0..file_size {
                file_blocks.push(FileBlock{
                    id: current_file_id, 
                    location: current_position
                });
                current_position += 1;
            }
            current_file_id += 1;
        } else {
            let free_space_size = c.to_string().parse::<i64>().unwrap();
            for _ in 0..free_space_size {
                file_blocks.push(FileBlock{
                    id: -1, 
                    location: current_position
                });
                current_position += 1;
            }
        }
    }
    file_blocks
}

fn defragment_disk(mut file_blocks: Vec<FileBlock>) -> Vec<FileBlock> {    
    file_blocks.sort_by(|a, b| a.location.cmp(&b.location));
    
    loop {
        let mut made_move = false;
        
        // Find first free space
        if let Some(free_space_pos) = file_blocks.iter().position(|block| block.id == -1) {
            // Find the rightmost non-free block
            if let Some(last_file_pos) = file_blocks.iter().rposition(|block| block.id != -1) {
                if last_file_pos > free_space_pos {
                    // Move just this one block
                    let file_id = file_blocks[last_file_pos].id;
                    file_blocks[free_space_pos].id = file_id;
                    file_blocks[last_file_pos].id = -1;
                    made_move = true;
                }
            }
        }
        
        if !made_move {
            break;
        }
    }
    
    file_blocks
}

fn defragment_disk_full_file(mut file_blocks: Vec<FileBlock>) -> Vec<FileBlock> {
    file_blocks.sort_by(|a, b| a.location.cmp(&b.location));
    
    // Find the highest file ID
    let max_file_id = file_blocks.iter()
        .map(|block| block.id)
        .max()
        .unwrap_or(-1);
    
    // Process files in descending order of file ID
    for file_id in (0..=max_file_id).rev() {
        // Count how many blocks this file uses
        let file_size = file_blocks.iter()
            .filter(|block| block.id == file_id)
            .count();
            
        // Find leftmost sequence of free spaces that can fit this file
        let mut best_free_space_start = None;
        let mut current_sequence_start = None;
        let mut current_sequence_length = 0;
        
        for (pos, block) in file_blocks.iter().enumerate() {
            if block.id == -1 {
                if current_sequence_start.is_none() {
                    current_sequence_start = Some(pos);
                }
                current_sequence_length += 1;
                
                if current_sequence_length >= file_size {
                    best_free_space_start = current_sequence_start;
                    break;  // Take the leftmost valid position
                }
            } else {
                current_sequence_start = None;
                current_sequence_length = 0;
            }
        }
        
        // If we found a suitable free space and it's to the left of the file
        if let Some(free_space_start) = best_free_space_start {
            let file_start = file_blocks.iter()
                .position(|block| block.id == file_id)
                .unwrap();
                
            if free_space_start < file_start {
                // Move the entire file to this free space
                let file_blocks_to_move: Vec<_> = file_blocks.iter()
                    .enumerate()
                    .filter(|(_, block)| block.id == file_id)
                    .map(|(i, _)| i)
                    .collect();
                
                // Clear original file locations
                for pos in &file_blocks_to_move {
                    file_blocks[*pos].id = -1;
                }
                
                // Place file in new location
                for i in 0..file_size {
                    file_blocks[free_space_start + i].id = file_id;
                }
            }
        }
    }
    
    file_blocks
}

fn calculate_file_checksum(file_blocks: Vec<FileBlock>) -> i64 {
    let mut checksum = 0;
    for (position, block) in file_blocks.iter().enumerate() {
        if block.id != -1 {
            checksum += (position as i64) * block.id;
        }
    }
    checksum
}