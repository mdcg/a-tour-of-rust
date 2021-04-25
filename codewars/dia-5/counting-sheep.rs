// https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust

// Minha solução
fn count_sheep(sheep: &[bool]) -> u8 {
    let mut count: u8 = 0;
    for s in sheep.iter() {
      if *s {
          count += 1;
        }
    }
    return count;
}

// Melhor solução
fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&x|x).count() as u8
}