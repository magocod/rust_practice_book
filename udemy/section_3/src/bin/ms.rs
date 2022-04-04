use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    let (tx, rx) = sync_channel(3);

    for n in 0..3 {
        // It would be the same without thread and clone here
        // since there will still be one `tx` left.
        let tx = tx.clone();
        // cloned tx dropped within thread
        thread::spawn(move || {
            println!("{}", n);
            // tx.send("ok").unwrap()
            tx.send(format!("{} {}", n, "ok")).unwrap()
            // tx.send(format_args!("{n} ok")).unwrap() // error
            // tx.send(concat!(n, "b")).unwrap() // error
        });
    }

    // Drop the last sender to stop `rx` waiting for message.
    // The program will not complete if we comment this out.
    // **All** `tx` needs to be dropped for `rx` to have `Err`.
    drop(tx);

    // Unbounded receiver waiting for all senders to complete.
    while let Ok(msg) = rx.recv() {
        println!("{}", msg);
    }

    println!("completed");
}
