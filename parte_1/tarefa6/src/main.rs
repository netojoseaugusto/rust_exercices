fn main() {
   println!("Por favor, digite uma sequencia de números reais: ");

   let mut input = String::new();

   std::io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

   let numbers: Vec<f64> = input
         .trim()
         .split_whitespace()
         .map( |x| x.parse::<f64>().expect("Por favor, insira numeros reais"))
         .collect();

   let mut sum: f64 = 0.0;

   for num in &numbers{
    sum = sum + num;
   }

   println!("A soma é {}", sum);

}

