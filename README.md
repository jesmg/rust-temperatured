# temperatured

Application for measuring the temperature of a Raspberry Pi, and sending email alerts when temperature is over 70ºC.

Temperature is going to be measured using the command `vcgencmd measure_temp`, available by default in Raspberry Pi OS.

The application will be run as a systemd timer.

## Is the application ready to use?

Yes, just compile it and set the following environment variables:

- `TD_EMAIL_TO`: Email address that is going to receive the alert.
- `TD_EMAIL_FROM`: Email address that is going to send the alert.
- `TD_EMAIL_SUBJECT`: Subject of the email alert.
- `TD_EMAIL_TEXT`: Body of the email alert.
- `TD_SMTP_SERVER`: Address of the SMTP server.
- `TD_SMTP_DOMAIN`: Fully qualified domain name corresponding to the email server.
- `TD_SMTP_USER`: SMTP server username.
- `TD_SMTP_PASS`: SMTP server password.

Or customize the code to meet your needs.

## Cross-compilation from Fedora 32

For cross-compilation install:

`cargo install cross`

And execute:

`cross rustc --target=armv7-unknown-linux-gnueabihf --release`

## Why the name temperatured?

The application has been named `temperatured` because it was originally created to be executed as a systemd timer.

## Contributors

* [Jesús Marín](https://github.com/jesmg): Rust fanboy
* [Ira Ramirez](https://github.com/Elvirarp92): Junior keyboard slammer