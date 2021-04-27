
use std::fs;

fn main() {
    
    //read input file contents into a string called contents
    let input = fs::read_to_string("input").expect("Unable to read file");
    let mut output_buffer = String::new();
    
    //Processing code here (use push or push_str to add to buffer contents)
    let input_vector:Vec<&str> = input.split_ascii_whitespace().collect();
    output_buffer.push_str(lloyd_clustering(input_vector).as_str());

    //compile buffer into a str and write in output file
    let output = output_buffer.as_str();
    fs::write("output", output).expect("Unable to write file");
}

fn lloyd_clustering(input:Vec<&str>) -> String {
    let mut final_centers = String::new();

    //parse k, n, and points
    let k = input[0].parse::<usize>().unwrap();
    let n = input[1].parse::<usize>().unwrap();
    let mut points = Vec::new();
    let mut index = 2;
    while index < input.len() {
        let mut point = Vec::new();
        for _i in 0..n {
            point.push(input[index].parse::<f64>().unwrap());
            index += 1;
        }
        points.push(point);
    }

    //initialize centers
    let mut centers = Vec::new();
    for i in 0..k {
        let mut point_copy = Vec::new();
        for j in 0..n {
            point_copy.push(points[i][j]);
        }
        centers.push(point_copy);
    }
    let mut old_centers = centers;

    //update centers
    loop {
        centers = get_centers(form_clusters(&points, &old_centers, n), n);

        if same_centers(&centers, &old_centers, k, n) {
            break;
        }

        old_centers = centers;
    }

    //parse centers
    for point in centers {
        for element in point {
            final_centers.push_str(format!("{} ", element).as_str());
        }
        final_centers.pop();
        final_centers.push_str("\n");
    }
    final_centers.pop();

    final_centers
}

fn same_centers(c1:&Vec<Vec<f64>>, c2:&Vec<Vec<f64>>, k:usize, n:usize) -> bool {
    for i in 0..k {
        for j in 0..n {
            if c1[i][j] != c2[i][j] {
                return false;
            }
        }
    }
    true
}

fn form_clusters(points:&Vec<Vec<f64>>, centers:&Vec<Vec<f64>>, n:usize) -> Vec<Vec<Vec<f64>>> {
    let mut cluster = Vec::new();
    for _i in 0..centers.len() {
        cluster.push(Vec::new());
    }
    for point in points {
        let mut min_center = 0;
        let mut min_distance = get_distance(point, &centers[0], n);
        for i in 1..centers.len() {
            if get_distance(point, &centers[i], n) < min_distance {
                min_center = i;
                min_distance = get_distance(point, &centers[i], n);
            }
        }
        let mut point_clone = Vec::new();
        for element in point {
            point_clone.push(*element);
        }
        cluster[min_center].push(point_clone);
    }
    cluster
}

fn get_distance(p1:&Vec<f64>, p2:&Vec<f64>, n:usize) -> f64 {
    let mut distance = 0.0;
    for i in 0..n {
        distance += (p1[i]-p2[i])*(p1[i]-p2[i]);
    }
    distance.sqrt()
}

fn get_centers(clusters:Vec<Vec<Vec<f64>>>, n:usize) -> Vec<Vec<f64>> {
    let mut centers = Vec::new();
    for cluster in clusters {
        let mut average = Vec::new();
        for i in 0..n {
            let mut sum = 0.0;
            for point in &cluster {
                sum += point[i];
            }
            average.push(sum/(cluster.len() as f64));
        }
        centers.push(average);
    }
    centers
}
