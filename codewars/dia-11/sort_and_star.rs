// https://www.codewars.com/kata/57cfdf34902f6ba3d300001e/train/rust

// Minha solução
fn two_sort(arr: &[&str]) -> String {
    let mut v: Vec<&str> = arr.iter().map(|x| *x).collect();
    v.sort();
    let mut cs: Vec<String> = Vec::new();
    for c in v[0].chars(){
        cs.push(c.to_string());
    }
    cs.join("***")
}

// Melhor solução
fn two_sort(arr: &[&str]) -> String {
    let mut v = arr.to_vec();
    v.sort();
    v[0].chars().map(|c| c.to_string()).collect::<Vec<String>>().join("***")
}

// Interessante..
use itertools::join;

fn two_sort(arr: &[&str]) -> String {
    join(match arr.iter().min() {
            None     => String::from(""),
            Some(&s) => String::from(s)
         }.chars(), "***")
}