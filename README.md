# hrtime
Human-Readable Time, stylized as `hrtime`, is a thin Rust library which
converts seconds into a colon-seperated time string, or vice versa.

## Why?
Seemed like a simple library to make as I am learning Rust and thought it
may prove useful to someone. As you can see by the versioning (0.1.0) it
is currently in very early stages.

## How?

### From seconds
This crate only contains three functions, `from_sec`, `from_sec_padded`,
and `to_sec`. The former two will convert a given `u64` into
colon-separated strings, with `from_sec_padded` specifically introducing
leading zeroes to reach the format `"00:00:00"` (HH:MM:SS). For example:
```rust
let secs = 123;
println!("{secs} seconds is {}", hrtime::from_sec(secs));
```
Will print `"123 seconds is 2:03"`!

and:
```rust
let secs = 234;
println!("{secs} seconds is {}", hrtime::from_sec_padded(secs));
```
Will print `"234 seconds is 00:03:54"`!

### To seconds
The third function is `to_sec`, which takes in a `time` string as an
argument and attempts to convert it into the amount of seconds it
represents. This string has some requirements to meet in order to work
though, like having the values separated by colons (ex: `"1:38"` for a
minute and 38 seconds) and there being no more than three colons
(ex: `"1:23:14:38"` will panic). `to_sec` may be used like so:
```rust
let time = "10:50";
let secs = hrtime::to_sec(time);
println!("{time} is {secs} seconds");
```
Which prints `"10:50 is 650 seconds"`!

### More
More examples can be seen in the
[examples](https://github.com/marcelohdez/hrtime/tree/master/examples)
folder.

## License
`hrtime` is licensed under the MIT license, for more information
please read the
[LICENSE file](https://github.com/marcelohdez/hrtime/blob/master/LICENSE).
