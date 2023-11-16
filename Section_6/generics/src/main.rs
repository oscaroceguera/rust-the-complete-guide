// struct Point<T, U>{
//     x: T,   // i32
//     y: T,   // i32
// }

trait Overview {
    fn overview(&self) -> String {
        String::from("This is a Rust course!")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course{
    fn drop(&mut self){
        println!("Dropping: {}", self.author);
    }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

fn main() {
    // let coord = Point {x: 5.0, y: 5.0};
    // let coord2 = Point {x: "x", y: 5.0};

    let course1 = Course{headline: String::from("Headline!"), author: String::from("Oscar")};
    let course2 = AnotherCourse{headline: String::from("Another Headline!"), author: String::from("Dulce")};

    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    // call_overview(&course1);
    // call_overview(&course2);
    // call_overview_other(&course1);
    // call_overview_other(&course2);

    drop(course1)
}

fn call_overview(item: &impl Overview){
    println!("Overview {}", item.overview());
}
// or
fn call_overview_other<T: Overview>(item: &T){
    println!("Overview {}", item.overview());
}
// fn overview(item1: &impl Overview, item2: &impl Overview)
// fn overview<T: Overview>(item1: &T, item2: &T)
// fn overview(item: &impl Overview + AnotherTrail)
// fn overview<T: Overview + AnotherTrail>(item1: &T, item2: &T)
