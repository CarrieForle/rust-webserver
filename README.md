# Rust-Webserver

A simple webserver implementation guided by [Rust Book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html) with minor tweaks.

Specifically,
- Most `unwrap()` is lifted to instead panic with error messages.
- Allow for 10 incoming requests instead of 4.
- There are doc comments in the public API.

# Run
Clone the repo and `cargo run` inside the repo.