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

## Time
`time` library was really strange to work with:
- [time.rs doc](https://time-rs.github.io/book/api/format-description.html)
- [time on docs.rs](https://docs.rs/time/latest/time/struct.Date.html#method.parse)
.. so I switched to `chrono`.

## Chrono
Used NaiveDate, as it seems to work all right for my little usecase.
- [chrono](https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html#method.checked_sub)


# Todo
- sort results
- display colors
- allow to add anniversaries from commandline (option)

# References
- [std::From](https://doc.rust-lang.org/std/convert/trait.From.html))
