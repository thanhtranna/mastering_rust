fn main() {
    let question = "How are you ?";
    // a &str type
    let person: String = "Bod".to_string();
    let namaste = String::from("नमस्ते");
    // unicodes yay!
    println!("{}! {} {}", namaste, question, person);
}
