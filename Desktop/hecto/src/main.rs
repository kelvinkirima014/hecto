use termion::raw::IntoRawMode;
use std::io::{ self,stdin,stdout, Read };
fn main() {
   
    println!("Hello, World!");
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in stdin().bytes(){
        let c = b.unwrap() as char;
        if c == 'q'{
            break;
        }

        println!("{}", c);
       
    }
}
