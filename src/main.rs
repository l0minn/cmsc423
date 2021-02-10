
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let mut input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    input.push('G');
    let mut min_locs = Vec::new();
    let mut min_skew = 0;
    let mut prev_skew;
    let mut running_skew = 0;
    let mut index = 0;

    for c in input.chars() {

        prev_skew = running_skew;
        if c == 'C' {
            running_skew -= 1;
        } else if c == 'G' {
            running_skew += 1;
        }

        if prev_skew < running_skew{
            if prev_skew < min_skew {
                min_locs.clear();
                if index != 0 {
                    min_locs.push(index);
                }             
                min_skew = prev_skew;
            } else if prev_skew == min_skew && index != 0 {
                min_locs.push(index);
            }
        }
        index += 1;
    }

    if min_locs.is_empty() {
        min_locs.push(1);
    }

    for i in min_locs{
        output_buffer.push_str(format!("{} ", i).as_str());
    }

    //compile buffer into a str and write in output file
    let output = output_buffer.trim_end();
    fs::write("output", output).expect("Unable to write file");
}
