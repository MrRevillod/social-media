
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

use common::constants::{
    PROJECT_EMAIL_SMTP_SERVER,
    PROJECT_EMAIL_ADDRESS,
    PROJECT_EMAIL_PASSWORD
};

fn main() {

    println!("Email smtp server: {}", *PROJECT_EMAIL_SMTP_SERVER);
    println!("Email address: {}", *PROJECT_EMAIL_ADDRESS);
    println!("Email password: {}", *PROJECT_EMAIL_PASSWORD);

    let message = Message::builder()
        .from(PROJECT_EMAIL_ADDRESS.to_string().parse().unwrap())
        .to("revillod2016luciano@gmail.com".parse().unwrap())
        .subject("subject".to_string())
        .body("aaa".to_string())
        .unwrap();
    
    let credentials = Credentials::new(
        PROJECT_EMAIL_ADDRESS.to_string(),
        PROJECT_EMAIL_PASSWORD.to_string()
    );

    let mailer = SmtpTransport::relay(&PROJECT_EMAIL_SMTP_SERVER)
        .unwrap()
        .credentials(credentials)
        .build();

    match mailer.send(&message) {
        Ok(_) => println!("Message sent"),
        Err(e) => println!("Error sending message: {}", e),
    }
}
