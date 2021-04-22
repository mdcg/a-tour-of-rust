// https://www.codewars.com/kata/55a70521798b14d4750000a4/train/rust

// Minha solução
fn greet(name: &str) -> String {
    let greeting = format!("Hello, {} how are you doing today?", name);
    return greeting;
}

// Melhor solução
fn greet(name: &str) -> String {
    format!("Hello, {} how are you doing today?", name)
}