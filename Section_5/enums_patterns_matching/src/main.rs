enum Pet {dog, cat, fish}

impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",
        }
    }
}

enum IpAddrKind{
    V4(String),
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let dog = Pet::dog;
    println!("{}", dog.what_am_i());

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopack = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Fish");
    what_pet("Cow");

    let dog2 = Some(Pet::cat);
    if let Some(Pet::dog) = dog2 {
        println!("The animal is a dog!");
    } else {
        println!("Not a dog!");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop(){
        println!("{}", top)
    }

    let x = 5;

    // match x {
    //     1 | 2 => println!("One or two!"),
    //     _ => println!("Not One or two!"),
    // }

    match x {
        1..=5 => println!("Matches"),
        _ => println!("Not matching"),
    }
}


// enum Option<T> {
//     None,
//     Some(T),
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn what_pet(input: &str) {
    match input {
        "Dog" => println!("I have a dog!"),
        "Cat" => println!("I have a cat!"),
        "Fish" => println!("I have a fish!"),
        _ => println!("I have no clue what pet yoy have"),
    }
}