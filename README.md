Proxy
=======

A learning project of writing a proxy in Rust. Currently only functional as a basic blocking TCP proxy.

Next steps
[ ] HTTP parsing
[ ] concurrency/async I/O
[ ] config file parsing
[ ] health checks
[ ] timeouts



### Dev process
window 1: `nc -lk 127.0.0.1 9000`
window 2: `cargo run`
window 3: `nc 127.0.0.1 8080` and proceed to enter messages and ensure they appear in window 1.