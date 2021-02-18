
use std::fs;
use howlong::*;

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

    //system clock
    let timer = HighResolutionTimer::new();

    /* Finding matching start positions */

    //compiling sp
    let mut sp:Vec<usize> = Vec::new();
    sp.push(0);

    for i in 1..target.len() {
        let mut temp = sp[i - 1];
        loop {
            if target[i] == target[temp] {
                sp.push(temp + 1);
                break;
            } else {
                if temp == 0 {
                    sp.push(0);
                    break;
                } else {
                    temp = sp[temp - 1];
                }
            }
        }
    }

    // for i in &sp {
    //     print!("{} ", i);
    // } 
    // println!("");

    //finding start indices
    let mut match_start_indeces = Vec::new();

    if genome.len() >= target.len() {
        let mut i = 0;
        let mut j = 0;
        while i < genome.len() {

            // println!(" {} {}", i, j);

            if genome[i] == target[j] {
                j += 1;

                if j == target.len() {
                    match_start_indeces.push(i  + 1 - target.len());
                    j = sp[target.len() - 1];
                }

                i += 1;
            } else {
                if j != 0 {
                    j = sp[j - 1];
                } else {
                    j = 0;
                    i += 1;
                }
            }
        }
    }

    //print elapsed time
    println!("{:?} have passed.", timer.elapsed());

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
