pub fn transpose_into_columns(lines: Vec<String>) -> Vec<Vec<char>> {
    if lines.is_empty() {
        return vec![];
    }

    // Find the maximum line length
    let max_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut columns: Vec<Vec<char>> = vec![Vec::new(); max_length];

    // Process each line
    for line in lines {
        // Pad shorter lines with spaces to maintain column alignment
        let padded: String = if line.len() < max_length {
            line.clone() + &" ".repeat(max_length - line.len())
        } else {
            line
        };

        // Add each character to its respective column
        for (col_idx, ch) in padded.chars().enumerate() {
            columns[col_idx].push(ch);
        }
    }

    // Clean up each column: trim whitespace and filter out empty columns
    columns
        .into_iter()
        .filter_map(|col| {
            let cleaned: Vec<char> = col
                .into_iter()
                .filter(|&c| !c.is_whitespace())
                .collect();
            if cleaned.is_empty() {
                None
            } else {
                Some(cleaned)
            }
        })
        .collect()
}