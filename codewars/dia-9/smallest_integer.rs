// https://www.codewars.com/kata/55a2d7ebe362935a210000b2/train/rust

// Minha solução
fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut b: Vec<&i32> = arr.iter().collect();
    b.sort();
    return *b[0];
}

// Melhor solução
fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}