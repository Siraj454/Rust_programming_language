use std::io;
use rand::Rng;
use std::collections::HashMap;
fn main() {
    //println!("Guess the number:");
    //let secret=rand::thread_rng().gen_range(1,7);
    println!("How many player wants to play");
    let mut player_total=String::new();
    io::stdin().read_line( &mut player_total)
    .expect("Failed to read line");
    let player_total: u32 = player_total.trim().parse().expect("Please type a number!");
    let mut players_list:Vec<String>=Vec::new();
    let mut number=1;
    while number <=player_total {
        println!("Enter player {} name",number);
        let mut player_names=String::new();
        io::stdin().read_line( &mut player_names)
        .expect("Failed to read line");
        players_list.push(player_names);
    
            number+=1;
                              }
    //println!("Players list is {:?}",players_list);
    //let mut die_value=roll_die();
    //println!("die rolled is {}",die_value);
   //loop{ player_turn(&players_list);

   
    player_turn(&players_list);
   }




 fn player_turn(names:&[String]){
  //let mut scores:std::collections::HashMap<_,_>=
  //names.into_iter().zip(init_score.into_iter()).collect();
  
  
        let name_len=names.len();
        
        for i in (0..name_len){
          let mut initial_scores:Vec<u8>=vec![0,0,0,0];
           println!("player {:?} turn",names[i]);
           println!("Press 1 and through die:");
           let mut play=String::new();
           io::stdin().read_line( &mut play)
           .expect("Failed to read line");
           let play: u32 = play.trim().parse().expect("Please type a number!");
           println!("Press 1 and through die:");
           if play ==1{
          
            let mut die:u8=rand::thread_rng().gen_range(1,7);
            println!("Die Rolled: {}",die);}
            
              initial_scores[i]+=die;

              println!("player {:?} score is {:?}",names,initial_scores);
           }
            //initial_scores[i]=initial_scores[i]+ die;
              
    }
  

          }
        