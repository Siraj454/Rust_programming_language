#[derive(Debug)]
enum IpAddrKind { //defined an enum
    V4,
    V6,
}
struct IpAddr { //created a struct with one argument of type IpAddrKind,other string
    kind: IpAddrKind,
    adress: String,
}
fn main()
{
    let home=IpAddr 
    {// created an instance of struct home
      kind : IpAddrKind::V4,
      adress : String::from("127.0.0.1"),
    };

let local=IpAddr //another instance of struct IpAddr
   {
       kind :IpAddrKind::V6,
       adress :String::from("::1"), 
   };
println!("Local adress is {:?} and version  is {:?}",local.adress,local.kind);
println!("Home adress is {:?} and version  is {:?}",home.adress,home.kind);
}

