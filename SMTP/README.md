To run this, you can do

    - cargo --test

This will run the test files

To test with custom Emails do:

First start the server
cargo build --release
./target/release/smtp_server

This will start server in port 2525

make sure you have telnet installed using brew in mac os or using sudo apt-get in linux

then run

telnet localhost 2525

Now you can send email, a test email is in src/example.md
