// Uma matriz é uma coleção de elementos de tamanho fixo onde todos os seus valores
// possuem o mesmo tipo.

// O tipo de dado para uma matriz é [T;N], onde T é o tipo dos valores e N é o 
// comprimento fixo conhecido em tempo de compilação.

// Os elementos podem ser recuperados individualmente com o operador [x], onde
// x é o indice do tipo usize (começando de 0) do elemento que deseja recuperar.

fn main() {
    let nums: [i32, 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}