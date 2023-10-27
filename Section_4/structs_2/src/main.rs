struct MyString<'a>{
    text: &'a str,
}

fn main() {
    let str1 = String::from("This is my string");
    let x = MyString{text: str1.as_str()};
}
