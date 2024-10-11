fn main() {
    let s1 = String::from("Olá"); // s1 é o proprietário da String
    let s2 = s1; // s1 agora não pode ser usada

     // println!("{}", s1); // Isso causaria um erro!
    println!("{}", s2); // Isso funciona
}