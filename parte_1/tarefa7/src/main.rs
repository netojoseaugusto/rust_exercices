fn calcula_medias(notas: &[f32]) -> f32{
    let tamanho = notas.len();
    let mut soma = 0.0;

    for nota in notas {
        soma += *nota;
    }

    return soma/tamanho as f32

}


fn main() {
    
    let notas = [10.0, 9.0, 7.5, 6.1, 7.2, 8.9, 7.6];
    let medias = calcula_medias(&notas); 
    println!("A média é {}", medias)

}
