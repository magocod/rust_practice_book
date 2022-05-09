extern crate dotenv;

use dotenv::dotenv;
use std::env;

use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header;
use lettre::transport::smtp::client::Tls;

fn main() {
    dotenv().ok();

    let example = env::var("EXAMPLE").expect("EXAMPLE must be set");
    println!("{}", example);


    let email = Message::builder()
        .from("smtp@domain.com".parse().unwrap())
        // .reply_to("other@domain.com".parse().unwrap())
        .to("example@domain.com".parse().unwrap())
        .subject("Hello Rust")
        .body(String::from("<!DOCTYPE html><html lang='en'><head><meta charset='UTF-8'><title>Title</title></head><body>Hello Rust</body></html>"))
        .unwrap();

    let creds = Credentials::new("fb49918a61595b".to_string(), "f41911cfabfaa7".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.mailtrap.io")
        .unwrap()
        .port(2525)
        .authentication(vec![Mechanism::Plain])
        .credentials(creds)
        .tls(Tls::None)
        .build();

    match mailer.test_connection() {
        Ok(v) => {
            println!("mailer.test_connection(): {}", v);
        }
        Err(e) => {
            println!("mailer.test_connection(): {}", e);
        }
    }

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
