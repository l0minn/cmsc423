
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    let lines:Vec<&str> = input.split('\n').collect();
    let sequence = lines[1];

    let mut sp:Vec<usize> = Vec::new();
    sp.push(0);

    for i in 1..sequence.len() {
        let mut temp = *sp.get(i - 1).unwrap();
        loop {
            if sequence.chars().nth(i) == sequence.chars().nth(temp) {
                sp.push(temp + 1);
                break;
            } else {
                if temp == 0 {
                    sp.push(0);
                    break;
                } else {
                    temp = *sp.get(temp - 1).unwrap();
                }
            }
            
        }
    }
    
    for i in sp {
        output_buffer.push_str(format!("{} ", i).as_str());
    }

    //compile buffer into a str and write in output file
    let output = output_buffer.trim_end();
    fs::write("output", output).expect("Unable to write file");
}
