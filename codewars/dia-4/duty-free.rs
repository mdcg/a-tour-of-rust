// https://www.codewars.com/kata/57e92e91b63b6cbac20001e5/train/rust

// Minha solução
fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    let price_with_discount = (price as f32) * (discount as f32)/100f32;
    return ((holiday_cost as f32)/price_with_discount) as i32;
}

// Melhor solução
fn duty_free(price: i32, discount: i32, holiday_cost: i32) -> i32 {
    holiday_cost *100   / (discount * price)
}