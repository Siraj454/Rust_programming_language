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
   cricket.insert("Pakistan",230);
   println!("cricket {:?}",cricket);
   cricket.entry("Pakistan").or_insert(500);
   cricket.entry("NewZeland").or_insert(150);


   for (team,score) in &cricket{
       println!("Team {}, Scores {}",team,score);
   }
   let mut sentence="samj samj k b jo na samjy usay na samj kehty hn";
   let mut map=HashMap::new();
   for word in sentence.split_whitespace(){
       let count=map.entry(word).or_insert(0);
       *count+=1;
   }
   println!("{:?}",map);
  
   // string 

  { let hellourdu = String::from("السلام عليكم");
    println!("Hello in urdu {:#?}",hellourdu);

   let mut first_name=String::from("siraj");
   let mut last_name=String::from(" Kashmiri");
   first_name.push_str(" Abbasi");
   println!("name is {}",first_name);
    let full_name=first_name+ ""+&last_name;
    println!("fullname is {}",full_name);

    //format macro 
    let s1=String::from("tic");
    let s2=String::from("tac");
    let s3=String::from("toe");
    let s=format!("{} {} {}",s1,s2,s3);
    println!("Formated game {}",s);
    let half_name=&full_name[0..5];
    println!("half name is {}",half_name);
    for c in full_name.chars(){
        println!(" {}",c);
    }
  }

}
