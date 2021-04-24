// https://www.codewars.com/kata/58649884a1659ed6cb000072/train/rust

// Minha solução
fn update_light(current: &str) -> String {
    match current {
        "green" => {"yellow".to_string()}
        "yellow" => {"red".to_string()}
        _ => {"green".to_string()}
    }
}

// Melhor solução
fn update_light(current: &str) -> String {
    match current {
        "green" => "yellow",
        "yellow" => "red",
        "red" => "green",
        _ => panic!()
    }.into()
}