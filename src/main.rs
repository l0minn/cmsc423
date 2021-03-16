
use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let blosum62 = fs::read_to_string("BLOSUM62.txt").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    let values:Vec<&str> = input.split_ascii_whitespace().collect();
    let s1 = values[0];
    let s2 = values[1];

    let blosum62_lines:Vec<&str> = blosum62.split('\n').collect();
    let col_heads:Vec<&str> = blosum62_lines[0].split_ascii_whitespace().collect();
    
    let mut blosum62_grid:HashMap<String, isize> = HashMap::new();
    for i in 1..blosum62_lines.len() {
        let row:Vec<&str> = blosum62_lines[i].split_ascii_whitespace().collect();
        for j in 1..row.len() {
            let key = format!("{}{}", row[0], col_heads[j-1]);
            let value = row[j].parse::<isize>().unwrap();
            blosum62_grid.insert(key, value);
        }
    }

    output_buffer.push_str(global_alignment(s1, s2, blosum62_grid, 5).as_str());

    //compile buffer into a str and write in output file
    let output = output_buffer.trim();
    fs::write("output", output).expect("Unable to write file");
}

fn global_alignment(s1:&str, s2:&str, score_grid:HashMap<String, isize>, indel_pen:isize) -> String{
    
    //construct dp_matrix
    let mut dp_matrix = Vec::new();
    for _i in 0..s1.len()+1 {
        let mut dp_line = Vec::new();
        for _j in 0..s2.len()+1 {
            dp_line.push(0);
        }
        dp_matrix.push(dp_line);
    }

    //initialize first row and column
    dp_matrix[0][0] = 0;
    for i in 1..dp_matrix.len() {
        dp_matrix[i][0]= dp_matrix[i-1][0] - indel_pen;
    }
    for i in 1..dp_matrix[0].len() {
        dp_matrix[0][i]= dp_matrix[0][i-1] - indel_pen;
    }

    //fill out dp_matrix
    for i in 1..dp_matrix.len() {
        for j in 1..dp_matrix[i].len() {
            let mut matched_letters = String::new();
            matched_letters.push_str(&s1[i-1..i]);
            matched_letters.push_str(&s2[j-1..j]);
            let from_top_left = dp_matrix[i-1][j-1] + score_grid.get(matched_letters.as_str()).unwrap();
            let from_top = dp_matrix[i][j-1] - indel_pen;
            let from_left = dp_matrix[i -1][j] -indel_pen;

            dp_matrix[i][j] = cmp::max(from_top_left, cmp::max(from_top, from_left));
        }
    }
    
    //print dp_matrix
    for line in &dp_matrix {
        for i in line {
            print!("{}\t", i);
        }
        println!();
    }

    //parse dp_matrix into score and strings
    let score = dp_matrix[dp_matrix.len() - 1][dp_matrix[dp_matrix.len() - 1].len() - 1];
    let mut matching_s1 = String::new();
    let mut matching_s2 = String::new();

    let mut i = s1.len();
    let mut j = s2.len();

    while i > 0 || j > 0 {
        //reached edge
        if i == 0 {
            matching_s1.insert(0, '-');
            matching_s2.insert(0, s2.chars().nth(j-1).unwrap());
            j -= 1;
        } else if j == 0 {
            matching_s1.insert(0, s1.chars().nth(i-1).unwrap());
            matching_s2.insert(0, '-');
            i -= 1;
        } 
        //hasn't reched edge yet
        else {
            //from top
            if dp_matrix[i][j] + indel_pen == dp_matrix[i-1][j] {
                matching_s1.insert(0, s1.chars().nth(i-1).unwrap());
                matching_s2.insert(0, '-');
                i -= 1;
            }
            //from left
            else if dp_matrix[i][j] + indel_pen == dp_matrix[i][j-1] {
                matching_s1.insert(0, '-');
                matching_s2.insert(0, s2.chars().nth(j-1).unwrap());
                j -= 1;
            }
            //from top left
            else {
                matching_s1.insert(0, s1.chars().nth(i-1).unwrap());
                matching_s2.insert(0, s2.chars().nth(j-1).unwrap());
                i -= 1;
                j -= 1;
            }
        }
    }

    //combine pared values into a single string
    let mut compiled_values = String::new();
    compiled_values.push_str(format!("{}\n", score).as_str());
    compiled_values.push_str(format!("{}\n", matching_s1).as_str());
    compiled_values.push_str(format!("{}\n", matching_s2).as_str());
    compiled_values
}
