
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("./input").expect("Unable to read file");
    
    //Processing code here (use push or push_str to add to buffer contents)

    panic!("{}", input);
}
