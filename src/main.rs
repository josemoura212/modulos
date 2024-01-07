mod operacoes;

use operacoes::matematica;

// use operacoes::matematica::somar;
// use operacoes::matematica::subtrair;
fn main() {
    println!("2 + 2 = {}\n", matematica::somar(2, 2));
    println!("2 - 2 = {}\n", matematica::subtrair(2, 2));
}
