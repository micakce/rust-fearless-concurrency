## Fearless concurrency in Rust, a practical example

In this post we are going to show an example of concurrency in action in rust. We are
going to show you with code and graphs what concurrency is and what happens at the
hardware level (a high level overview).

The program we are going to use is a simplified version of grep that counts how many times
a word appears in a file. We are going to do so in a sequencial/single threaded way, and
of course, in a multithreaded way.


