
fn tem_caracteres_unicos(input: &str) -> bool {
    let mut conjunto_de_caracteres = [false; 128];
    for &c in input.as_bytes(){
        let indice = c as usize;
        println!("Caracter {}, indice {}", c as char, indice);

        if conjunto_de_caracteres[indice] {
            println!("Caracter duplicado encontrado");
            return false
        } 
        conjunto_de_caracteres[indice] = true;    
    }
    println!("Nenhum caracter duplicado encontrado");
    return true
}

fn main() {
    
    let teste = "Capito";
    tem_caracteres_unicos(teste);

}
