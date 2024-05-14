fn eh_permutacao(str1: &str, str2: &str) -> bool {

    if str1.len() != str2.len(){
        return false;
    }

    let mut contagem_caracteres = [0; 128];

    for &c in str1.as_bytes(){
        contagem_caracteres[c as usize] += 1;
    }

    for &c in str2.as_bytes(){
        contagem_caracteres[c as usize] -= 1;
        if contagem_caracteres[c as usize] < 0 {
            return false;
        }
    }

    return true
}


fn main() {
    let str1 = "abc";
    let str2 = "cbd";

    if eh_permutacao(str1, str2){
        println!("As strings são permutacoes");
    } else{
        println!("As strings não são permutacoes");
    }

}
