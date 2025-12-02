# AOC 2025

---

My solutions for [advent of code 2025](https://adventofcode.com/2025).

This year I'll be doing it in rust again.

---

### How to run:

Use ``cargo run -- -h`` to see all options.

Use ``cargo run -- --day=1 --part=1 solve`` or ``cargo run -- -d1 -p1`` to run a specific day and part,

Use ``cargo run -- all`` to run all days and see how fast they all get solved.

When day isn't specified or 0 it will default to the current day of the month but only in december.

Use ``cargo run -- -d1 fetch-input`` to get the input and load it in the correct place.

Use ``cargo run -- -d1 fetch-all-input`` to get the inputs for all days up until the specified day which aren't already downloaded.

To use fetch input you must have a `.env` file in the root folder containing a session token, you can get one by searching through the requests under the network tab in your browser (I don't know about a better method).

```env
SESSION="your_token_here"
```

The cli will also tell you how long it took to run the code, and if it was on debug or release mode

You can also use ``cargo test`` to run the test with the provided test cases.
When I'm developing I like to use ``bacon`` to automatically run the tests while programming,
this does require the [bacon](https://crates.io/crates/bacon) crate.

---

### My thoughts on each day

A bit of text describing my experience with creating a solution for a specific day.

* day 1: It's only the first day and I already have to debug a wrong solution with a correct test.

* day 2: I feel like there is a really smart solution for today, too bad I didn't find it.

---

Thanks for reading!