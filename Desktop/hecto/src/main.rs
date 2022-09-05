fn main() {
    use std::io::{ self, Read };
    println!("Hello, World!");

    for b in io::stdin().bytes(){
        let c = b.unwrap() as char;

        println!("{}", c);
       
    }
}
