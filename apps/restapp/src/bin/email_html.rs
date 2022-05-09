use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::transport::smtp::client::Tls;
use lettre::{Message, SmtpTransport, Transport};

fn main() {
    // The html we want to send.
    let html = r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Hello from Rust!</title>
    </head>
    <body>
        <div style="display: flex; flex-direction: column; align-items: center;">
            <h2 style="font-family: Arial, Helvetica, sans-serif;">Hello from Rust!</h2>
            <h4 style="font-family: Arial, Helvetica, sans-serif;">A mailer library for Rust</h4>
        </div>
    </body>
    </html>"#;

    let email = Message::builder()
        .from("smtp@domain.com".parse().unwrap())
        // .reply_to("other@domain.com".parse().unwrap())
        .to("example@domain.com".parse().unwrap())
        .subject("Hello Rust")
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Hello from Rust! A mailer library for Rust")), // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
        )
        .expect("failed to build email");

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
