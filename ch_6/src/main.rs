#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("value of 4 {:?}",four);
    println!("ip value for V4 is {:?}",route(IpAddrKind::V4));
    route(IpAddrKind::V6);


}
fn route(ip_kind:IpAddrKind) {}

