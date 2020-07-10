use std::fs::File;
use std::io::{self,Read};
fn main(){
    fn read_username_from_file()-> Result<String ,io::Error>{
    let f= File::open("hello.txt");
    let mut f =match f{
        Ok(file)=>file,
        Err(error)=>return Err(error)
    };
    let mut s= String::new();
    match f.read_to_string(&mut s){

        Ok(_)=>Ok(s),//usize can also be wriiteen
        Err(error)=>Err(error),
    }
}
println!("{:#?}",read_username_from_file());
}