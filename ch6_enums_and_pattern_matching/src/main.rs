/* Enumerations

Enums allow for defining a type by enumerating its possible variants. Enums can encode meaning along with data.
A useful enum (called Option) can express a value being something or nothing (like Swift optionals?).
Pattern matching with the match expression can make it easy to run different code for different values of an enum.
The `if let` construct is another convenient and conise idiom available to handle enums.

*/

enum IpAddressKind {
    V4,
    V6,
}

// fn route(ip_kind: IpAddressKind) {

// }

fn main() {
    // Note: Enum variants are namespaced under its identifier, using a double colon to separate the two
    let four: IpAddressKind = IpAddressKind::V4;
    let six: IpAddressKind  = IpAddressKind::V6;

    // route(IpAddressKind::V4);
    // route(IpAddressKind::V6);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
