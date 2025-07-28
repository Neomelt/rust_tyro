// #![allow(unused)]
// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     fn route(ip_type: IpAddrKind) {}

//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
// }

#![allow(unused)]
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
