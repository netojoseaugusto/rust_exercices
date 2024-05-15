fn compress_string(s: &str) -> String {
    let mut compressed = String::new();
    let mut count: i32 = 0;

    for (i, c) in s.chars().enumerate() {
        count += 1;

        if i + 1 >= s.len() || c != s.chars().nth(i+1).unwrap() {
            compressed.push(c);
            compressed.push_str(&count.to_string());
            count = 0;
        }
    }

    if compressed.len() >= s.len(){
        return s.to_string();
    }

    

    return compressed;


}

fn main() {
    let original_str = "aabccccdddd";
    let compressed_str = compress_string(original_str);

    println!("Original: {}", original_str);
    println!("Compressed: {}", compressed_str);
}
