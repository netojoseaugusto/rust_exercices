use std::io;

fn main() {

    println!("Digite o número de inteiros que você quer somar:");

    let mut nr_inteiros = String::new();
    io::stdin().read_line(&mut nr_inteiros).expect("Error when reading number 1");

    let itens: i32 = nr_inteiros.trim().parse().expect("Insira um número inteiro");

    let mut contador: i32 = 1;

    let mut soma: i32 = 0;

    while contador <= itens {

        let mut entrada = String::new();

        io::stdin().read_line(&mut entrada).expect("Error when reading entrada");
        let entrada_num: i32 = entrada.trim().parse().expect("Insira um número inteiro");
        soma = soma + entrada_num;
        contador = contador + 1;

    }

    println!("O resultado total é: {}", soma);
    
    }

