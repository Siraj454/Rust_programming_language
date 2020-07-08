use std ::collections::HashMap;
//[#[derive(debug)]];
fn main() {
    println!("Storing values in list and arrays");
    let mut names=vec![String::from("siraj"),String::from("ali"),String::from("ahmed")];
    let age=vec![20,30,40,];
    let mut name_age:HashMap<_,_>=names.into_iter().zip(age.into_iter()).collect();
    println!("names with ages {:?}",name_age);
   println!("Siraj age is  {:?}",name_age.get("siraj"));

   
   let mut cricket = HashMap::new();
   cricket.insert("England",200);
   println!("cricket {:?}",cricket);
}
