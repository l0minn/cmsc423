
use std::fs;

struct Tree {
    head_node_index: usize,
    nodes: Vec<Node>
}

struct Node {
    value: Option<String>,
    parent_index: Option<usize>,
    children_indices: Vec<usize>
}

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    output_buffer.push_str(create_suffix_tree(input.as_str()).compile_edges().as_str());

    //compile buffer into a str and write in output file
    let output = output_buffer.as_str();
    fs::write("output", output).expect("Unable to write file");
}

fn create_suffix_tree(text:&str) -> Tree {
    let mut suffix_tree = Tree::new();

    for i in 0..text.len() {
        suffix_tree.add_suffix(&text[i..]);
    }

    suffix_tree
}

impl Tree {
    pub fn new() -> Tree {
        let mut tree = Tree {
            head_node_index: 0,
            nodes: Vec::new()
        };
        tree.nodes.push(Node::new());
        tree
    }

    fn add_suffix(&mut self, text:&str) {
        self.add_suffix_helper(self.head_node_index, text);
    }

    fn add_suffix_helper(&mut self, parent_index:usize, text:&str) {

        //check for matching first character
        let mut matching_index = None;
        for index in self.nodes.get(parent_index).unwrap().get_children() {
            if self.nodes.get(index).unwrap().value.as_ref().unwrap()[0..1] == text[0..1] {
                matching_index = Some(index);
                break;
            }
        }

        match matching_index {
            //add new node with text as its value
            None => {
                self.nodes.push(Node {
                    value: Some(text.to_string()),
                    parent_index: Some(parent_index),
                    children_indices: Vec::new()
                });
                let new_index = self.nodes.len()-1;
                self.nodes.get_mut(parent_index).unwrap().children_indices.push(new_index);
            },
            //add new node at mismatch
            Some(index) => {
                let mut text_index = 1;
                let node_text = self.nodes.get(index).unwrap().get_value();
                //compare other characters
                while text_index < node_text.len() && text_index < text.len() {
                    //keep looking through until mismatch
                    if node_text[text_index..text_index+1] == text[text_index..text_index+1] {
                        text_index += 1;
                    } 
                    //insert new node at mismatch location
                    else {
                        //create new node and add to "tree"
                        let mut new_node = Node {
                            value: Some(text[..text_index].to_string()),
                            parent_index: Some(parent_index),
                            children_indices: Vec::new()
                        };
                        new_node.children_indices.push(index);
                        let new_index = self.nodes.len();
                        let new_new_node = Node {
                            value: Some(text[text_index..].to_string()),
                            parent_index: Some(new_index),
                            children_indices: Vec::new()
                        };                        
                        let new_new_index = self.nodes.len()+1;
                        new_node.children_indices.push(new_new_index);
                        self.nodes.push(new_node);
                        self.nodes.push(new_new_node);

                        //add link from parent node to this node
                        self.nodes.get_mut(parent_index).unwrap().children_indices.push(new_index);

                        //remove link from parent node to old child node
                        let mut target_index = 0;
                        for i in 0..self.nodes.get(parent_index).unwrap().get_children().len() {
                            if self.nodes.get(parent_index).unwrap().get_children().get(i).unwrap() == &index {
                                target_index = i;
                                break;
                            }
                        }
                        self.nodes.get_mut(parent_index).unwrap().children_indices.remove(target_index);

                        //set new node as the parent of the old child node
                        self.nodes.get_mut(index).unwrap().parent_index = Some(new_index);

                        //change value of old child
                        self.nodes.get_mut(index).unwrap().value = Some(String::from(&node_text[text_index..]));
                        
                        return;
                    }
                }
                //recurse with new text and parent, since end of text had not been reached
                if text_index < text.len() {
                    self.add_suffix_helper(index, &text[text_index..]);
                }
            }
        }
    }

    fn compile_edges(&self) -> String {
        let mut compiled_edges = String::new();

        for node in &self.nodes {
            match &node.value {
                None => {},
                Some(v) => {
                    compiled_edges.push_str(format!("{}\n", v).as_str());
                }
            }
        }

        compiled_edges
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            value: None,
            parent_index: None,
            children_indices: Vec::new()
        }
    }

    fn get_value(&self) -> String {
        let mut copy = String::new();
        match &self.value {
            None => {},
            Some(s) => {
                copy.push_str(s);
            }
        }
        copy
    }

    fn get_children(&self) -> Vec<usize> {
        let mut copy = Vec::new();
        for index in &self.children_indices {
            copy.push(index.clone());
        }
        copy
    }
}
