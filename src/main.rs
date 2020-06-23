extern crate lettre;
extern crate lettre_email;

use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use std::process::Command;
use std::env;

fn main() {
    let temperature = get_temperature();
    if temperature > 70 {
        send_mail()
    }
}

fn get_temperature() -> u8 {
    let output = Command::new("bash")
    .arg("-c")
    .arg("vcgencmd measure_temp | egrep -o '[0-9]*\\.[0-9]*'")
    .output()
    .expect("failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let temperature: f32 = stdout.trim().parse().unwrap();
    let temperature = temperature as u8;
    temperature
}

fn send_mail() {
    let email_to = env::var("TD_EMAIL_TO").unwrap();
    let email_from = env::var("TD_EMAIL_FROM").unwrap();
    let email_subject = env::var("TD_EMAIL_SUBJECT").unwrap();
    let email_text = env::var("TD_EMAIL_TEXT").unwrap();

    let smtp_server  = env::var("TD_SMTP_SERVER").unwrap();
    let smtp_domain = env::var("TD_SMTP_DOMAIN").unwrap();
    let smtp_user = env::var("TD_SMTP_USER").unwrap();
    let smtp_pass = env::var("TD_SMTP_PASS").unwrap();

    let email = Email::builder()
        .to(email_to)
        .from(email_from)
        .subject(email_subject)
        .text(email_text)
        .build()
        .unwrap();

    let mut mailer = SmtpClient::new_simple(&smtp_server).unwrap()
    .hello_name(ClientId::Domain(smtp_domain))
    .credentials(Credentials::new(smtp_user, smtp_pass))
    .smtp_utf8(true)
    .authentication_mechanism(Mechanism::Plain)
    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).transport();

    let result = mailer.send(email.into());

    result.unwrap();
}
