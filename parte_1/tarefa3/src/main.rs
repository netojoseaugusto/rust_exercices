fn e_primo(num: i32) -> bool{

    if num <= 1{
        return false;
    }

    let limite = (num as f32).sqrt() as i32 + 1;

    for i in 2..limite{
        if num % i == 0{
            return false;
        }
    }

    return true
} 

fn main() {

    let numero = 13;
    let resultado = e_primo(numero);
    println!("O numero {} e primo? {}", numero, resultado);

}
