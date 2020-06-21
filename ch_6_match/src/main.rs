enum Color{
    Red,
    Green,
    Blue,
    Dark,
}
fn currency_value (note:Color)->i32{
match  note{
  Color::Red => 100,
  Color::Green => 500,
  Color::Blue => 1000,
  Color::Dark => 10,
}
 }

fn main() {
    let red_note=currency_value(Color::Red);
    let blue_note=currency_value(Color::Blue);
    let green_note=currency_value(Color::Green);
    let dark_note=currency_value(Color::Dark);
    println!("lets check the color and and its currency value");
    println!("The Value of Red color note is {},blue is {} green is {} and Dark is {}",
    red_note,blue_note,green_note,dark_note);
    


      let s1 = "hello dolly".to_string();
      dump(&s1);
      println!("s1 {}", s1);
     
      println!("Parsing String to Number");//remember data type should be same
      let parsed: i32 = "5".parse().unwrap();
      let turbo_parsed = "10".parse::<i32>().unwrap();
  
      let sum = parsed + turbo_parsed;
      println!("Sum: {:?}", sum);
      

      println!("For loop check ");
      let a= 5;
      let mut b=0;// mut is used to take value outSIde
      for i in 0..a{
         b=b+1;
        println!("value of b in scope is {}",b)
      }
     println!(" ***value of b outScope is {}",b);

  }
  


fn dump(s: &String) {
  println!("{}", s);
}
