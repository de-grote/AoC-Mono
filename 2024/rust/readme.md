# AOC 2024

---

My solutions for [advent of code 2024](https://adventofcode.com/2024).

This year I'll be doing it in rust again.

---

### How to run:

Use ``cargo run -- -h`` to see all options.

Use ``cargo run -- --day=1 --part=1 solve`` or ``cargo run -- -d1 -p1`` to run a specific day and part,

Use ``cargo run -- all`` to run all days and see how fast they all get solved.

When day isn't specified or 0 it will default to the current day of the month but only in december.

Use ``cargo run -- -d1 fetch-input`` to get the input and load it in the correct place.

To use fetch input you must have a `.env` file in the root folder containing a session token, you can get one by searching through the requests under the network tab in your browser (I don't know about a better method).

```env
SESSION="your_token_here"
```

The cli will also tell you how long it took to run the code, and if it was on debug or release mode

You can also use ``cargo test`` to run the test with the provided test cases.
When I'm developing I like to use ``cargo watch -x test`` to automatically run the tests while programming,
this does require the [cargo watch](https://crates.io/crates/cargo-watch) crate.

---

### My thoughts on each day

A bit of text describing my experience with creating a solution for a specific day.

* day 1: I accidentally pasted some code into the testcase which caused the parser to give a partial result, which lost me a lot of time today. Skill issue.

* day 2: I spend way too long trying some clever solution, and then I brute forced it and it worked.

* day 3: Should've just added regex tbh, recursive parsers are also *something* I guess.

* day 4: I love pattern matching.

---

Thanks for reading!