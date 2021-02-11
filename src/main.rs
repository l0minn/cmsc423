
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    let mut skew_vals = Vec::new();
    let mut running_skew = 0;
    for c in input.chars() {
        if c == 'C' || c == 'G' || c == 'A' || c == 'T' {
            if c == 'C' {
                running_skew -= 1;
            } else if c == 'G' {
                running_skew += 1;
            }
            skew_vals.push(running_skew);
        }
    }

    let mut min_skew = skew_vals[0];
    let mut min_locs = Vec::new();

    for i in 0..skew_vals.len() {
        if skew_vals[i] < min_skew {
            min_locs.clear();
            min_locs.push(i + 1);
            min_skew = skew_vals[i];
        } else if skew_vals[i] == min_skew {
            if min_locs.is_empty() {
                min_locs.push(i + 1);
            } else {
                let temp = min_locs.pop().unwrap();
                if temp == i - 1 {
                    min_locs.push(i + 1);
                } else {
                    min_locs.push(temp);
                    min_locs.push(i + 1);
                }
            }
        }
    }

    for i in min_locs {
        print!("{} ", i);
        output_buffer.push_str(format!("{} ", i).as_str());
    }
    println!();

    //compile buffer into a str and write in output file
    let output = output_buffer.trim_end();
    fs::write("output", output).expect("Unable to write file");
}
