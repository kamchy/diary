# Parse diary-like file
This tool is my quick reminder for people's anniversaries and birthdays and
a small Rust exercise.

It reads stdin, expecting anniversary lines: a date folowed by a  whitespace, followed by a description.
For example, if this is the contents of `anni.txt` file:

```
1953-03-16 Richard Stallman's birthday
1960-12-28 Linus Torvalds' birthday
1956-01-31 Guido van Rossum's birthday
1961-07-04 Brendan Eich birthday
```

then after you build:
```bash
cargo build --release
```
and execute the binary with:
```bash
cat anni.txt | ./target/release/diary
````
you will see following output:
```
Today (2026-02-08): 
36 days till #73 anniversary of [Richard Stallman's birthday] (1953-03-16)
323 days till #66 anniversary of [Linus Torvalds' birthday] (1960-12-28)
357 days till #71 anniversary of [Guido van Rossum's birthday] (1956-01-31)
146 days till #65 anniversary of [Brendan Eich birthday] (1961-07-04)

```

# Libraries


## Chrono
For date parsing and time operations I use NaiveDate, as it seems to work all right for my little usecase.
- [chrono](https://docs.rs/chrono/latest/chrono)


# Todo
- sort results
- display colors
- allow to add anniversaries from commandline (option)

# References
- [std::From](https://doc.rust-lang.org/std/convert/trait.From.html)
- [rust by example](https://doc.rust-lang.org/rust-by-example/hello.html)
- [rust book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
