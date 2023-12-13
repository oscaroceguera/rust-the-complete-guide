// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use std::collections::BinaryHeap;
// use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // let mut nums: Vec<i32> = vec![];

    // nums.push(1);
    // nums.push(2);
    // nums.push(3);

    // let pop = nums.pop();

    // println!("{:?}", pop);

    // let two = nums[1]; // copy
    //                    // &nums[1], creates a reference if copy is not available
    // println!("{}", two);

    // let one = nums.first(); // return an Option<T>, so None if vec is empty, or Some<T> is [0]
    // println!("{:?}", one);

    // // .last
    // // .first_mut and .last_mut, so will borrow mutable references

    // println!("{}", nums.len()); // return a value of the length
    // println!("{}", nums.is_empty()); // bool

    // nums.insert(0, 10);
    // nums.insert(3, 12);
    // nums.insert(2, 25);

    // nums.remove(3);

    // nums.sort();
    // println!("{:?}", nums);

    // nums.reverse();
    // println!("{:?}", nums);

    // nums.shuffle(&mut thread_rng());
    // println!("{:?}", nums);

    // let mut bheap = BinaryHeap::new();

    // bheap.push(1);
    // bheap.push(18);
    // bheap.push(20);
    // bheap.push(5);

    // bheap.pop();

    // println!("{:?}", bheap);
    // println!("{:?}", bheap.peek()); // peek is going to return Option<T>, return None if empty, or Some(T) otherwise
    // println!("{:?}", bheap);

    // let mut hm = HashMap::new();

    // hm.insert(1, 1);
    // hm.insert(5, 2);
    // hm.insert(30, 3);
    // let old = hm.insert(30, 4);

    // println!("{:?}", hm);
    // println!("{:?}", old);

    // println!("{}", hm.contains_key(&30));
    // println!("{:?}", hm.get(&5));

    // let one = hm.remove(&1); // remove the value
    // println!("{:?}", one);

    // let remove = hm.remove_entry(&5); // remove the key and the value
    // println!("{:?}", remove);

    // hm.clear();

    // println!("{}", hm.is_empty());

    let mut hs = HashSet::new();

    // len()
    // is_empty()

    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);

    // hs.remove(&2);

    for x in hs.iter() {
        println!("Iter: {}", x);
    }

    let mut hs2 = HashSet::new();

    hs2.insert(1);
    hs2.insert(3);
    hs2.insert(5);
    hs2.insert(7);

    for x in hs.intersection(&hs2) {
        println!("Intersection: {}", x);
    }

    let intesection = &hs & &hs2; // shorthand intersection using the binary bitwise & operator

    for x in intesection {
        println!("Short hand may: {}", x);
    }

    let union = &hs | &hs2;
    for x in union {
        println!("Union: {}", x);
    }

    let diff = &hs - &hs2;

    for x in diff {
        println!("Diff: {}", x);
    }
}
