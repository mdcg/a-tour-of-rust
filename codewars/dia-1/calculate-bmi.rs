// https://www.codewars.com/kata/57a429e253ba3381850000fb/train/rust

// Minha solução
fn bmi(weight: u32, height: f32) -> &'static str {
    // https://doc.rust-lang.org/std/primitive.f32.html#method.powf
    let bmi = (weight as f32 / (height.powf(2f32)));
    if bmi <= 18.5f32 {
        return "Underweight";
    } else if bmi <= 25.0f32 {
        return "Normal";
    } else if bmi <= 30.0f32 {
        return "Overweight";
    } else {
        return "Obese";
    }
}

// Melhor solução
fn bmi(weight: u32, height: f32) -> &'static str {
    let index = weight as f32 / height.powi(2);
    match index {
        index if index <= 18.5 => "Underweight",
        index if index <= 25.0 => "Normal",
        index if index <= 30.0 => "Overweight",
        _ => "Obese",
    }
}
