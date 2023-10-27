// struct User {
//     active: bool,
//     username: String,
//     sing_in_count: u32,
// }

// struct Coordinates(i32,i32,i32);

// struct UnitStruct;

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn whats_my_width(&self) -> u32 {
        self.width
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width
    }
}

fn main() {
    // let user1 = User{active: true, username: String::from("Oscar"), sing_in_count: 0};
    // println!("{}", user1.username);


    // let user2 = build_user(String::from("Eduardo"));
    // println!("{}", user2.username);

    // let cords = Coordinates(1,2,3);

    let mut sq = Square {width: 5, height: 5};
    println!("{}", sq.area());
    println!("{}", sq.whats_my_width());
    sq.change_width(10);
    println!("{}", sq.whats_my_width());
}


// fn build_user(username: String) -> User {
//     User {
//         username,
//         active: true,
//         sing_in_count: 1,
//     }
// }