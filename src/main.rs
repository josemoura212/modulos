mod operacoes;

fn main() {
    println!("2 + 2 = {}\n", operacoes::matematica::somar(2, 2));
    println!("2 - 2 = {}\n", operacoes::matematica::subtrair(2, 2));
}
