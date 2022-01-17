fn main() {
    println!("Hello, world!");
}


struct IpAddr {
    kind: IpAddrKind,
    value: String
}
struct IpAddrFour {}
struct IpAddrSix {}

enum IpAddrKind {
    V4,
    V6
}

enum SIpAddrKind {
    V4(String),
    V6(String)
}

enum STIpAddrKind {
    V4(u32, u32, u32, u32),
    V6(String)
}

enum StrengthIpAddr {
    V4(IpAddrFour),
    V6(IpAddrSix)
}

fn routes(ip: &IpAddrKind)-> &IpAddrKind {
    ip
}

fn f1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let a_four = routes(&four);
    let a_six = routes(&six);
    let home = IpAddr {
        kind: IpAddrKind::V4,
        value: String::from("127.0.0.1")
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        value: String::from("::1")
    };
    let a_home = SIpAddrKind::V4(String::from("127.0.0.1"));
    let a_loopback = SIpAddrKind::V6(String::from("::1"));
    let b_home = STIpAddrKind::V4(127, 0, 0, 1);
    let b_loopback = STIpAddrKind::V6(String::from("::1"));
}