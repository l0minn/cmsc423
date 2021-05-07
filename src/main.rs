
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    let input_vector:Vec<&str> = input.split_ascii_whitespace().collect();
    output_buffer.push_str(hierarchical_clustering(input_vector).as_str());

    //compile buffer into a str and write in output file
    let output = output_buffer.as_str();
    fs::write("output", output).expect("Unable to write file");
}

fn hierarchical_clustering(input:Vec<&str>) -> String {
    //set up axis and grid
    let size:usize = input[0].parse().unwrap();

    let mut axis = Vec::new();
    for i in 1..(size+1) {
        axis.push(format!("{}",i));
    }

    let mut max_val = 0.0;
    let mut grid = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();
        for j in 0..size {
            max_val = f64::max(max_val,input[i*size+j+1].parse::<f64>().unwrap());
            row.push(input[i*size+j+1].parse::<f64>().unwrap());
        }
        grid.push(row);
    }

    //loop and merge
    let mut clustering = String::new();
    while axis[0].len() < size*2-1{
        //find min distance greater than 0
        let mut x = 0;
        let mut y = 0;
        let mut min_val = max_val+1.0;

        for i in 0..size {
            for j in i+1..size {
                if (grid[i][j] != 0.0) && (grid[i][j] < min_val) {
                    x = i;
                    y = j;
                    min_val = grid[i][j];
                }
            }
        }
        if x == 0 && y == 0 {
            break;
        }

        //update clustering
        let new_cluster = String::from(format!("{} {}\n",axis[x].as_str(), axis[y].as_str()).as_str());
        clustering.push_str(new_cluster.as_str());
        
        //update grid
        let x_labels:Vec<&str> = axis[x].split_ascii_whitespace().collect();
        let y_labels:Vec<&str> = axis[y].split_ascii_whitespace().collect();
        let x_weight:f64 = x_labels.len().to_string().parse().unwrap();
        let y_weight:f64 = y_labels.len().to_string().parse().unwrap();
        let divisor:f64 = x_weight + y_weight;
        let mut avg_distances = Vec::new();
        for i in 0..size {
            if i == x || i == y {
                avg_distances.push(0.0);
            } else {
                avg_distances.push((grid[x][i]*x_weight+grid[y][i]*y_weight)/divisor)
            }
        }
        for i in 0..size {
            grid[x][i] = avg_distances[i];
            grid[i][x] = avg_distances[i];
            grid[y][i] = 0.0;
            grid[i][y] = 0.0;
        }

        //update axis
        axis[x] = format!("{} {}", axis[x].as_str(), axis[y].as_str());
        axis[y] = String::from("0");
    }

    clustering
}
