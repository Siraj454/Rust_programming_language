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
     //if num then ok otherwise err  ,ok (p) will continue (if players less then 1  ) else {p} returned

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

break players  };
println!("players are {:?}",players );


let mut players_list:Vec<String>= Vec::new();
let mut turn_scores=HashMap::new();
//loop from 1 to num of  players(loop players return num of players want to take part)
//like 1..4
for turn in 1..=players{
  println!("Enter name of  player {}",turn);
  let mut player=String::new();
  io::stdin().read_line(&mut player).expect("Enter name only");
  //add entry to the list 
  players_list.push(player.trim().to_string());
  //add entry to the hashmap and assign value of 0 to score ,u8 type
  turn_scores.entry(player.trim().to_string()).or_insert(0u8);

  }
println!("players list {:?}",players_list);
println!("players Scores {:?}",turn_scores);
//println!("players are {}",players)
//generate random number for each turn and check conditions
   let mut turn = 1;
  loop 
  {
       for myturn in 1..=players{
         let mut score:u8=0;
         let dice=rand::thread_rng().gen_range(1,7);
         println!("player turn  {}",turn);
         match dice{
           6=>{
             score=score+dice;
             println!("its six");
             let dice=rand::thread_rng().gen_range(1,7);
             match dice{
               6=>{
                     println!("2nd Six");
                     score=score+dice;
                     let dice=rand::thread_rng().gen_range(1,7);
                     match dice{
                       6=>score={
                        println!("\nAlas! Turn {} Dice Roll of Player {} - {} is 0 due to 3 Sixer and Total {}",turn,myturn,&players_list[(myturn-1) as usize],turn_scores.get(&players_list[(myturn-1)as usize]).unwrap());
                         0
                       },
                       _=>score=score+dice
                     }
               },
               _=> score=score+dice
             }

           },
           _=> score =score+dice

         }
     
       
      
     
      println!("turn score {}",score);
      let myscore=match turn_scores.get(&players_list[(myturn-1) as usize]) {
        Some(data) => data,
        None => &0,
      };
      let playerturn = format!("{}{}",players_list[(myturn-1)as usize],turn);
      println!("score map {:?}",turn_scores);
      println!("my scorea {:?}",myscore)
  } 
       
     turn = turn + 1;
    if turn >4{
      break
    }}
    
}


