fn main() {
    let name: &str = "Jose";
    let mut age = 43;
    age += 1;
    println!("Hello {}!", name);
    println!("Voce tem {} anos", age);

    //let x: i64 = 23;
    //let f: f32 = 6.7;
    //let b: bool = true;

    let number1 = 24;
    let number2 = 42;

    if number1 > number2 {
        println!("{} > {}", number1, number1);
    }
    else {
        println!("{} <= {}", number1, number2);
    }

}
