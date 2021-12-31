fn main() {
    let mut s = String::from("Hi, I'm a string");
    modify_str(&mut s);
    let sub = &s[0..3];
    print_str(s); // s will be invalid after this.
}

fn modify_str(str_to_modify: &mut String) {
    str_to_modify.push_str("sdflkj")
}

fn print_str(str_to_print: String) {
    println!("{}", str_to_print);
}
