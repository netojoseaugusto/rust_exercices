fn dobro(num: i32) -> i32{
    return 2*num
}

fn maximo(a: i32, b: i32) -> i32{

    if a > b {
        return a;
    }
    else {
        return b;
    }

}

fn main() {
    println!("O dobre de 5 é {}", dobro(5));
    println!("{} é maior", maximo(10, 15));
}
