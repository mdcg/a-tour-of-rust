// https://www.codewars.com/kata/5861d28f124b35723e00005e/train/rust

// Minha solução
fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    if mpg * gallons >= distance_to_pump {
        return true;
    } else {
        return false;
    }
}

// Melhor solução
fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    distance_to_pump <= mpg * gallons
}
