use std::io::{self, BufRead};

/// The layer's weights per hidden unit. These are *transposed* compared to the other layers so that
/// they align nicely with the biases and layer inputs.
#[allow(clippy::unreadable_literal)]
#[allow(clippy::excessive_precision)]
const WEIGHTS: [[f32; 5]; 5] = [
    [1.227929, -0.16913611, 0.01634032, -0.65211457, -0.20668325],
    [0.64210439, -0.67459118, 0.09487623, 1.21616852, 0.78601265],
    [
        1.01473153,
        0.64515805,
        -0.22562766,
        -0.61605215,
        -0.11170049,
    ],
    [1.3138932, -0.43405721, 0.25345135, 0.0328721, -0.39521286],
    [
        -0.42810276,
        -0.08025748,
        0.72735178,
        -0.12146949,
        0.22173114,
    ],
];

#[allow(clippy::unreadable_literal)]
#[allow(clippy::excessive_precision)]
const BIASES: [f32; 5] = [0.34330517, 0.51000923, 0.4419407, -0.03040748, -0.04246865];

fn relu(x: f32) -> f32 {
    x.max(0.0)
}



fn forward(inputs: &[f32]) -> Vec<f32> {
    inputs
        .iter()
        .zip(&WEIGHTS)
        .zip(&BIASES)
        .map(|((input, weights), bias)| {
            let output: f32 = weights.iter().map(|weight| weight * input).sum();

            relu(output + bias)
        })
        .collect()
}

fn main() {
    // Read line from stdin
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    let inputs: Vec<f32> = line
        .split_whitespace()
        .map(|s| s.parse().expect("Could not parse input"))
        .collect();
    let outputs: Vec<String> = forward(&inputs)
        .into_iter()
        .map(|output| output.to_string())
        .collect();

    println!("{}", outputs.join(" "));
}
