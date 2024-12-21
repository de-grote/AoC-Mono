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

Use ``cargo run -- -d1 fetch-all-input`` to get the inputs for all days up until the specified day which aren't already downloaded.

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

* day 5: Today I'm grateful sorting is trivial to do in rust.

* day 6: Run this one on release mode if you value your time.

* day 7: Easiest day so far if you know recursion.

* day 8: I still don't know what people were cooking with gcd today.

* day 9: Part 2 is so much more difficult than part 1.

* day 10: Easier than it looked like I'd be.

* day 11: Slept through my alarm, rip competitive time.

* day 12: Didn't sleep through my alarm, got 689th for part 2.

* day 13: Linear algebra and floats are scary.

* day 14: Press y if you see a christmas tree, you're the algorithm now!

* day 15: Now I can finally say I've made sokoban.

* day 16: First time implementing Dijkstra this year, should've seen it coming.

* day 17: Delta time of 14 hours, I'm so good. Also it was hardcoded, but I fixed it later.

* day 18: Easy puzzle for my birthday, also I know I could optimise part 2, but it's fine.

* day 19: I really liked todays puzzle, even though it was kind of generic.

* day 20: I got the right idea instantly, just spend way too much time debugging when I didn't had to.

* day 21: I procrastinated a lot on starting today, for good reasons.

---

Thanks for reading!