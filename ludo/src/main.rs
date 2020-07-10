use std::io;
use rand::Rng;
use std::collections::HashMap;
fn main(){
  println!("Siraj's Snake Ludo");
  // num of players input
  let players=loop{
  println!("How many players Want to take part");
  let mut players=String::new();
   io::stdin().read_line(&mut players).expect("Enter number only");
   //convert players string to int type
   let players:u8=match players.trim().parse(){
     Ok(p)=>{
       if p<2{
         println!("Add more then one Player");
         continue
         }
         else{p}},

     Err(_)=>{ println!("Only numbers,Try Again!");
     continue
},
};
break players};

let mut players_list:Vec<String>= Vec::new();
let mut scores=HashMap::new();
for player in 1..=players{
  println!("Enter name of  player {}",player);
  let mut player=String::new();
  io::stdin().read_line(&mut player).expect("Enter name only");
  
  players_list.push(player.trim().to_string());
  scores.entry(player.trim().to_string()).or_insert(0u8);

}
println!("players list {:?}",players_list);
println!("players Scores {:?}",scores);
}