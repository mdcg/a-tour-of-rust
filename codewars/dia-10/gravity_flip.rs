// https://www.codewars.com/kata/5f70c883e10f9e0001c89673/train/rust

// Minha solução
fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut v: Vec<u32> = cubes
        .iter()
        .map(|&x| x)
        .collect();
    
    v.sort();
    
    if dir == 'L' {
        v.reverse();
    }

    return v;
}


// Melhor solução
fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut res = cubes.to_vec();
    res.sort();
    if dir == 'L' { res.reverse(); }
    res
}

// Interessante...
use itertools::Itertools;

fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let cmp: fn(&u32, &u32) -> std::cmp::Ordering = match dir {
        'R' => |a, b| a.cmp(b),
        'L' => |a, b| b.cmp(a),
        _ => unreachable!()
    };
    cubes.iter().cloned().sorted_by(cmp).collect()
}