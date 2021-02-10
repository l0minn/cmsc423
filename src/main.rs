
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)

    /* Parsing input */
    let mut target = Vec::new();
    let mut genome = Vec::new();

    let mut passed_first_newline = false;
    for c in input.trim().chars() {
        if !passed_first_newline {
            if c == '\n' {
                passed_first_newline = true;
                continue;
            } else if c == 'G' || c == 'C' || c == 'A' || c == 'T'{
                target.push(c);
            }
        } else if c == 'G' || c == 'C' || c == 'A' || c == 'T' {
            genome.push(c);
        }
    }

    // /* testing parsing */
    // for c in target {print!("{}", c);}
    // println!("");
    // for c in genome {print!("{}", c);}
    // println!("");

    /* Finding mathing start positions */
    let mut match_start_indeces = Vec::new();

    if genome.len() >= target.len() {
        for i in 0..=(genome.len() - target.len()) {
            let mut matches_target = true;
            for j in 0..target.len() {
                if target[j] != genome[i + j] {
                    matches_target = false;
                    break;
                }
            }
            if matches_target {
                match_start_indeces.push(i);
            }
        }
    }

    // /* testing start postion search */
    // for i in match_start_indeces {print!("{} ", i);}
    // println!("");

    /* Parse match_start_indeces into a output buffer */

    for i in match_start_indeces {
        output_buffer.push_str(format!("{} ", i).as_str());
    }
    
    //compile buffer into a str and write in output file
    let output = output_buffer.trim_end();
    fs::write("output", output).expect("Unable to write file");
}
