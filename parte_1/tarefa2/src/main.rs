fn maior_number(numeros: &[i32]) -> i32 {
    let mut maior = numeros[0];

    for num in numeros {
        if &maior < num{
            maior = *num;
        }
    }

    return maior
}

fn main() {
    let numeros = [1,2,3,4,5,10,32,44342,5435];

    let maior = maior_number(&numeros);

    println!("O maior numero é {}", maior);
}
