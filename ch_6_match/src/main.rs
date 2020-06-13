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
    
}
