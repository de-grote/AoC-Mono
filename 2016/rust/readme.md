# AoC 2016 in rust

I was trying to find some challanges to work on to improve my rust skills,
and this was basically the most fun thing for me to do.

I chose 2016 specifically since I have already done most of 2021 and 2015 in python.

I might try to finish those one day but that's neither here nor there.

I'm also going to _try_ and finish all this year but I expect it to become really difficult to make an algorithm that finishes in a reasonable amount of time (multiple minutes) around day 20.

---

### Run

I didn't make any CLI or anything similar so if you want to run it replace the day and part number with the one you want to solve.

```rust
day01::part1::solve().expect("invalid input");
```

If you want your own input replace `input.txt` with your input and recompile.

---

### My thoughts on each day

A bit of text describing my expierence with creating a solution for a specefic day.

* day 1: A lot of small bugs today, mostly type mismatches with different types of ints, understandable but annoying.

* day 2: It honestly took me way to long to make the HashMap because of all the braces, went pretty smoothly apart from that though.

* day 3: Pretty fun, went smoothly, too bad my solution for part 2 is a _bit_ verbose, but its pretty fast so its fine.

* day 4: Nothing wrong with the first part but the second part is really annoying to manually check all lines to see if it's the one I need because it doesn't tell me what I'm looking for.

* day 5: Brute force hashing, how fun. Also it took me half an hour to debug this because no one told me ``>>`` has priority over ``&`` and I almost went insane. This day also made me install a dependency and takes way to long to run in debug mode.

* day 6: I tried way to long to see if I could get my code to compile using the lenght of one line, but after way too much time I just put in 8 and gave up on that, the rest of the day was doable and part 2 was just changing a max to a min so that was easy.

* day 7: Gotta love regex (and debugging). Also fancy-regex > regex.

* day 8: Pretty doable, and part 2 wasn't really harder than part 1, also why is there no ``Default::default()`` for large arrays?

* day 9: Doable if it wasn't for me using the worst names possible for variables, also gotta love recursion over impure functions.

* day 10: Loved the moment when I realised all the commands had to be executed when possable instead of in order and I had to rewrite all my code.

---

Thanks for reading!