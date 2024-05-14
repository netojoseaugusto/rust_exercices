use std::io;

fn main() {
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("Error when reading number 1");

    let num: i32 = number1.trim().parse().expect("Insira um número inteiro");

    for i in 1..11{
        println!("{} x {} = {}", num, i, i * num);
    }
}


