
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let processed_inpput:String = input.split_ascii_whitespace().collect();
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)

    /* sample code */

    //compile buffer into a str and write in output file
    let output = output_buffer.as_str();
    fs::write("output", output).expect("Unable to write file");
}
