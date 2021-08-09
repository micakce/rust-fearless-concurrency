## Fearless concurrency in Rust, a practical example

#### Introduction

In this post, we are going to show an example of concurrency in action in Rust. We are
going to show you with code and graphs what concurrency is and what happens at the
hardware level (a high level overview).

The program we are going to use is a simplified version of `grep` (`minigrep`) that
counts how many times a word appears in a file. We are going to do so in a single-threaded way, and of course, in a multi-threaded way.

#### Project structure

The scaffolding follows Rust's basic project structure:
```bash
.
├── books # contains files to be read
├── src # source code files
│   ├── main.rs
│   └── lib.rs
├── Cargo.lock
├── Cargo.toml
└── README.md
```

The structure of the program is simple; we have a `Config` struct whose function is to
verify that the correct arguments are passed to the program:

- **query:** The word to look for in the file
- **dir:** The folder containing all the files to search through
- **concurrently:** Environment variable used to either run the program single or multi
  threaded

And two functions, `search_and_count` and `search_and_count_concurrently`. So we read the
folder, read each file in it, and search for the `query` indicated as an argument. The
difference between the two functions is that in one, we split the file (read as a string)
into 4 strings and we search through each of them concurrently.



#### Program in action

We are using the `time` linux command to get time and CPU of the execution of the program.

```bash
# by default it runs without concurrency
❯ time cargo run how books
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-fearless-concurrency how books`
Running program single thread
The word "how" appears 16160 time in file "books/pride_and_prejudice.txt"
The word "how" appears 13552 time in file "books/the_flag_of_the_adventurer.txt"
The word "how" appears 1936 time in file "books/report_of_migration_of_birds.txt"
cargo run how books  14.95s user 0.13s system 99% CPU 15.088 total

```

![Single Threaded minigrep](./images/single_thread.gif)

As you can see, there is only one CPU (thread) spiking at all times, and this makes sense because
our program is running single-threaded. But why is it switching from one CPU to the
other, isn't it supposed to run in only one of them?


Now let's run our program with the concurrency option:


```bash
# we use the flag CONCURRENTLY to activate concurrency
❯ time CONCURRENTLY=1 cargo run how books
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-fearless-concurrency how books`
Running program multi threaded
The word "how" appears 16160 time in file "books/pride_and_prejudice.txt"
The word "how" appears 13552 time in file "books/the_flag_of_the_adventurer.txt"
The word "how" appears 1936 time in file "books/report_of_migration_of_birds.txt"
CONCURRENTLY=1 cargo run how books  15.60s user 0.20s system 365% CPU 4.324 total
```

![Multi Threaded minigrep](./images/multi_thread.gif)


Wow, wow. What was that? Yes, the graph was all over the place and our program executed
faster than before. If we pay attention to the terminal output the `time` commands gave
us some information about the execution of the program, we care about the part that says
`system 365% CPU 4.324 total`, compared to the `system 99% CPU 15.088 total` from our
previous execution, we can see it executed almost in 1/4 of the time.  Remember we are
splitting each file in 4 equal parts and searching through them at the same time? That's
what you see in the graph, 4 threads spiking at all times, reading each part of the file
in parallel, taking almost 1/4 of the time, and this makes sense because we are doing 4
times the work, 365% compare to only 99%.


### References:

- Source code [here](https://github.com/micakce/rust-fearless-concurrency)
- Books taken from [Gutenberg](https://www.gutenberg.org/)
- Resources monitoring tool: [Bottom](https://github.com/ClementTsang/bottom)



