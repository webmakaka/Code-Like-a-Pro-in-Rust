fn print_string(s: String) {
    println!("print_string: {}", s);
}

fn print_str(s: &str) {
    println!("print_str: {}", s);
}

fn main() {
    // let s: str = "impossible str";
    print_string(String::from("String"));
    print_str(&String::from("String"));
    print_str("str");
    // print_String("str"); error! does not compile
}
