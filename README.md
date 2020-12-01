# Advent of Code 2020

My attempt at Advent of Code for 2020. This is the first year I've attempted it, and I'm looking forward to using this as an opportunity to learn Rust.

https://adventofcode.com/2020

# Objectives

* Learn the basics of Rust, because I think it's a cool language and I want to try building stuff with it
* Sharpen my algorithmic problem-solving skills, because this isn't something I get much practice in day-to-day
* Most importantly: have fun!

# Structure

Each day's puzzle will be solved by a separate binary, named `day1`,
`day2`, etc. However, as there will no doubt be reused code across
binaries, there will also be a single library with multiple modules
to contain this.

Each day's binary has its own unit tests. Once I start refactoring,
the library will also no doubt have its own tests. Perhaps at some
point I might see the need for some kind of integration test, but
I think this is unlikely.
