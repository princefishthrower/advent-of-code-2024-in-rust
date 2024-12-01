use crate::utils::read_as_columns::read_as_columns;

pub fn run_a() -> std::io::Result<()> {
    // get the two columns as lists of integers
    let columns = read_as_columns("src/day_1/input.txt")?;
    let column1 = columns[0].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let column2 = columns[1].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    // sort the columns
    let mut sorted_column1 = column1.clone();
    sorted_column1.sort();
    let mut sorted_column2 = column2.clone();
    sorted_column2.sort();

    // find the absolute value difference between the two columns at each index
    let mut total_distance = 0;
    for i in 0..sorted_column1.len() {
        total_distance += (sorted_column1[i] - sorted_column2[i]).abs();
    }

    // print the total distance
    println!("Total distance: {}", total_distance);

    Ok(())
}

pub fn run_b() -> std::io::Result<()> {
    let columns = read_as_columns("src/day_1/input.txt")?;
    let column1 = columns[0].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let column2 = columns[1].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    // for each number in column1, find how many times that number appears in column2. this becomes the 'similarity score' for that number and it is multipled by the number itself
    let mut total_similarity_score = 0;
    for num in column1 {
        let similarity_score = column2.iter().filter(|&n| *n == num).count();
        total_similarity_score += similarity_score as i32 * num;
    }

    // print the total similarity score
    println!("Total similarity score: {}", total_similarity_score);

    Ok(())
}