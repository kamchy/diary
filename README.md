
# Parse diary-like file


# Libraries

## Time
`time` library was too strange and difficult:
- [time.rs doc](https://time-rs.github.io/book/api/format-description.html)
- [time on docs.rs](https://docs.rs/time/latest/time/struct.Date.html#method.parse)

## Chrono
I switched to `chrono`:
- [chrono](https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html#method.checked_sub)


# Todo
- better error handling (messages)
- use From for errors (See [std::From](https://doc.rust-lang.org/std/convert/trait.From.html))
