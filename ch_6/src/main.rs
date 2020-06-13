#[derive(Debug)]
enum IpAddr { 
    //defined an enum
    V4(String),
    V6(String),
}
fn main()
{
    let home = IpAddr::V4(String::from("192.183.3"));
    let local = IpAddr::V6(String::from("::1"));
    println!("Adress is {:?} ",home) ;
    println!("v6 Adress is {:?}",local);

}
