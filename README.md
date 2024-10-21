I am learning rust. To learn it most efficiently, I am - solving problems here https://practice.course.rs/compound-types/enum.html - and following this video to clarify concepts https://www.youtube.com/watch?v=BpPEoZW5IiY&t=14214s

I finished till chapter 6 and then started working on projets. I know concepts like generics and other memory management stuff from Java, Solitidy so I skipped to building stuff. If you don't have these basics covered, I would highly recommend you finish the practise book first.

I like to build projects while learning so here are some projects I implemented in this repo

## SMTP

    - SMTP: I learned to build a [SMTP server in Rust](https://notes.eatonphil.com/handling-email-from-gmail-smtp-protocol-basics.html) by following this tutorial by phil eaton https://notes..

This is in Go so you can't just copy it. I used chat gpt to convert some parts of it in Rust and a lot of it I wrote on my own

## PNGME

To finish this project, I had to understand the concepts till chapter 11. Some concepts like Option, Result, Packages, crates etc, I learned along the way without directly refering the book

    - pngme: Png me is a intermediate level rust project. It teaches you how to endcode and decode data into png files. The guide simply guides you through the process but the exact implementation is left to you. The author has provided some test cases to nudge you in the expected direction

## Lending Pool Yield Optimization

fill in these values in .env

```
PRIVATE_KEY=
NETWORK_RPC=
```

Use the following command to watch for changes and automatically run the project:

```
cargo watch -q -c -w src/ -x run
```
