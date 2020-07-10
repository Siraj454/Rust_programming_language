use std::fs::File;
use std::io::{self,Read};
fn main(){
    fn read_username_from_file()->Result<String ,io::Error>
    let mut f =match f{
        let f= File::open("hello.txt");
        ok(file)=>file,
        Err(error)=>return Err(error)
    };
    let mut s= String::new();
    match f.read_to_string(&mut s){

        ok(_)=>ok(s); //usize can also be wriiteen
        Err(e)=>e,
    }
    println!("{:?}",read_username_from_file);
}