# maid.felix
[WIUT intranet](https://intranet.wiut.uz) timetable scraper.

## How to run
To run the scraper, first make sure you have [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html#install-rust-and-cargo) installed. Create an `.env` file with two values for `LOGIN` and `PASSWORD` (your [intranet](https://intranet.wiut.uz) login and password). Finally, simply run the following from the project directory:

```shell
cargo run --release
```

## NAQ (Never Asked Questions)

### Why was this written?
In order to build applications to make university life more bearable, there was a need for timetable data. Since we weren't granted (and probably will never be granted) access to university's API, scraping was the only reasonable option left. The scraper should not overwhelm the university server as the scraper is set to sleep between requests and is only meant to be run once a day to keep the data up-to-date.

### I found a discrepancy in the timetable data. What should I do?
Feel free to either open a new issue in this repo or leave a message in [mad-maids group chat](https://t.me/madmaids_wiut).

## License
`maid.felix` is [licensed](https://github.com/mad-maids/maid.felix/blob/main/LICENSE) under the terms of the GNU General Public License v3.0.
