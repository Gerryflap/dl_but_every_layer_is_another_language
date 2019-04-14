use std::io::{self, BufRead};

const WEIGHTS: [[f32; 5]; 5] = [[ 1.227929  ,  0.64210439,  1.01473153,  1.3138932 , -0.42810276],
       [-0.16913611, -0.67459118,  0.64515805, -0.43405721, -0.08025748],
       [ 0.01634032,  0.09487623, -0.22562766,  0.25345135,  0.72735178],
       [-0.65211457,  1.21616852, -0.61605215,  0.0328721 , -0.12146949],
       [-0.20668325,  0.78601265, -0.11170049, -0.39521286,  0.22173114]];

const BIASES: [f32; 5] = [ 0.34330517,  0.51000923,  0.4419407 , -0.03040748, -0.04246865];

fn relu(x: f32) -> f32 {
    if x < 0.0 {
        return 0.0
    } else {
        return x
    }
}

fn forward(vec: Vec<f32>) -> Vec<f32> {
    let mut output: Vec<f32> = Vec::new();
    let mut o: f32 = 0.0;
    for j in 0..BIASES.len() {
        o = 0.0;
        for i in 0..WEIGHTS.len() {
            o += vec[i] * WEIGHTS[i][j]
        }
        o += BIASES[j];
        o = relu(o);
        output.push(o);
    }
    return output
}

fn main() {
    // Read line from stdin
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let num_strings = line.split_whitespace();
    let vec = num_strings.map(|s: &str| s.parse().unwrap()).collect::<Vec<f32>>();

    let out_str: Vec<String> = forward(vec).iter().map(ToString::to_string).collect();

    println!("{}", out_str.join(" "));
}
