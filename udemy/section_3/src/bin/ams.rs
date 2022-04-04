use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // Create a shared channel that can be sent along from many threads
    // where tx is the sending half (tx for transmission), and rx is the receiving
    // half (rx for receiving).
    let (tx, rx) = channel();
    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("tx n:{}", i);
            tx.send(i).unwrap();
        });
    }

    for n in 0..10 {
        println!("rx n:{}", n);
        let _ = rx.recv().unwrap();
        // assert!(0 <= j && j < 10);
    }
}
