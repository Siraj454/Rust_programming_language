#[derive(Debug)]
struct User{
    username:String,
    email:String,
    active:bool,
    sign_in_count:u64,

}
//************************main fn starts here****************//
fn main() {
    //make mutable to change the values inside the struct
    let mut _user_1=User{
        username:String::from("Siraj"),
        email:String::from("electrical.uet24@gmail.com"),
        active:true,
        sign_in_count:12,
    };
    
    _user_1.email=String::from("siraj@123.com");//email has been changed 
    println!("User Details {:#?}",_user_1);

    //structure with in function
    //feild init shortend
    
    //print area of function
    let rectangle_lenght:u32=5;
    let rectangle_width:u32=4;
    let area=area(rectangle_lenght,rectangle_width);
    println!("area of the rectangle with simple arguments is {}",area);
    
    //tuple rectangle 
    let rect1=(6,4);
    println!("area of rectangle with tuple arguments is {}",tup_rect(rect1));

    //STRUCT RECTANGLE 
     let rect_1=Rectangle{
         lenght:10,
         width:4,
     };
    //pass struct to function
     println!("Area of rectangle with struct arguments is {}",rect_area(&rect_1));

     //*********************method ***********//
     let student_1=Student{
         name:String::from("Siraj Abbasi"),
         roll_no:24,
         class:String::from("EE-17b")

     };
     println!("method output");
     println!( "student {:?}",student_1.student_info());

}
//*************************main ends here***************************************//
//**********comparison of struct with functions********// 
fn area(lenght:u32,width:u32)-> u32 {
    let area:u32=lenght*width;
    area

}
// *****do same with tuple,it is little more appropriate******//
fn tup_rect(dimension:(u32,u32))->u32 {
    dimension.0*dimension.1
}

//****************make a structure of ractangle to make more meaningful*********
struct Rectangle{
    lenght:u8,
    width:u8,
}
fn rect_area(rectangle:&Rectangle)->u8 {
    rectangle.lenght*rectangle.width

}
//&&&&&&&&&&&&&&&&&&&&&&&&&&&&___METHOD___&&&&&&&&&&&&&&&&&&&&&&//
//Methods ,it is same as python classes ,self argument ,and instances
//it starts with a struct and method takes info from struct
struct Student{
    name:String,
    roll_no:u32,
    class:String,
}
impl Student{
    fn student_info(&self){
        println!("student name is {}, roll_no is {}, class {}",self.name,self.roll_no,self.class);
    }
}


