mod operacoes;

use operacoes::matematica as mat;

// use operacoes::matematica::somar;
// use operacoes::matematica::subtrair;
// use operacoes::matematica::{somar,subtrair};

fn main() {
    println!("2 + 2 = {}\n", mat::somar(2, 2));
    println!("2 - 2 = {}\n", mat::subtrair(2, 2));
}
