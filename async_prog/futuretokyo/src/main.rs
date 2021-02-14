use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
    message: String
}

impl Delay {

    fn add_message(&mut self, text: &str) {
        self.message.push_str(text);
    }

}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("{}", "Hello world".to_owned() + ", my message: " + &self.message);
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let mut future = Delay { when, message: String::new() };

    future.add_message("example");
    let out = future.await;
    assert_eq!(out, "done");
}
