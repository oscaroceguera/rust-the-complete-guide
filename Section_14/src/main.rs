use std::sync::mpsc;

fn main() {
    let (transmitter, receiver) = mpsc::channel(1000);
    let tx = transmitter.clone();

    // let val = String::from("Transmitting!");
    // std::thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // let msg = receiver.recv().unwrap();
    // println!("{}", msg);

    std::thread::spawn(move || {
        let vec = vec![
            String::from("Transmitting"),
            String::from("from"),
            String::from("Original"),
        ];

        for val in vec {
            transmitter.send(val).unwrap();
        }
    });
    std::thread::spawn(move || {
        let vec = vec![
            String::from("Clone"),
            String::from("Is"),
            String::from("Transmitting"),
        ];

        for val in vec {
            tx.send(val).unwrap();
        }
    });

    for rec in receiver {
        println!("{}", rec)
    }
}
