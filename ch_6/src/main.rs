#![allow(unused_variables)]
use::std::fmt::Debug;
enum IpAddr { 
    //defined an enum
    V4(String),
    V6(String),
}
struct IpAdr{
    kind:IpAdrKind,
    adress:String,

}
enum IpAdrKind{
    V4,
    V6,
}

fn main()
{

    let home = IpAddr::V4(String::from("192.183.3"));
    let local = IpAddr::V6(String::from("::1"));
    println!("Adress  {:?} ",home) ;
    println!("Adress  {:?}",local);

    //***********enum inside struct ***************/
    struct IpAdr{
        kind:IpAdrKind,
        adress:String,

    }
    let v4=IpAdr{
        kind:IpAdrKind::V4,
        adress:String::from("192.181.2.2"),
    };
   println!("version 4 info {:?}",v4.adress);
   let my_message=Message::Write(String::from("this section is bit tricky"));
   println!("{:?}",my_message)
   
}
//***************************MAIN ENDS HERE***************//
//each variant of enum can have different types,i.e (string,bool,struct,enum)

//***********enum  inside enum**********
enum Message{
    Move{x:i32,y:i32},
    Quit,
    ChangeColor(i32,i32,i32),
    Write(String),
}
//*******now write all struct which are called inside enum
//*******this is same as above code */
struct MoveMessage{
    x:i32,
    y:i32,
}
struct QuitMessage{}//unit struct 
struct ChangeColorMessage(i32,i32,i32);//tuple struct 
struct WriteMessage(String);
