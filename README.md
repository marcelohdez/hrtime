# hrtime
Human-Readable Time, stylized as `hrtime`, is a thin Rust library which
converts seconds into either a colon-seperated time string and vice versa,
or into the raw hour, minute, and second values.

## Why?
Seemed like a simple library to make as I am learning Rust and thought it
may prove useful to someone. As you can see by the versioning (0.2.0) it
is currently in very early stages.

## How?

### From seconds
This crate only contains four functions, two "from" functions: `from_sec`
and `from_sec_padded`, and two "to" functions: `to_sec` and `to_time`.
The two "from" functions will convert a given `u64` into a colon-separated
time string, with `from_sec_padded` specifically introducing leading
zeroes to reach the format `"00:00:00"` (HH:MM:SS). An example using
`from_sec`:
```rust
let secs = 234;
println!("{secs} seconds is {}", hrtime::from_sec(secs));
```
Will print `"123 seconds is 3:54"`,

and the same example but using `from_sec_padded`:
```rust
let secs = 234;
println!("{secs} seconds is {}", hrtime::from_sec_padded(secs));
```
Will print `"234 seconds is 00:03:54"`!

### To seconds
The first "to" function is `to_sec`, which takes in a `time` string as an
argument and attempts to convert it into the amount of seconds it
represents. This string has some requirements to meet in order to work
though, like having the values separated by colons (ex: `"1:38"` for a
minute and 38 seconds) and there being no more than three colons (ex:
`"1:23:14:38"` will panic). `to_sec` may be used like so:
```rust
let time = "10:50";
let secs = hrtime::to_sec(time);
println!("{time} is {secs} seconds");
```
Which prints `"10:50 is 650 seconds"`!

### To time
The second "to" function is `to_time`, which returns the raw hour, minute,
and second values represented by the given `u64` respectively. This is
returned as a tuple containing a `u64` (hour), and two `u8`s (minute and
seconds) more information can be seen in its documentation. `to_time` may
be used like so:
```rust
let seconds = 650;
let (hrs, min, sec) = hrtime::to_time(seconds);
println!("{seconds} seconds is {hrs}h{min}m{sec}s");
```
Which prints `"650 seconds is 0h10m50s"`!

### More
More examples can be seen in the
[examples](https://github.com/marcelohdez/hrtime/tree/master/examples)
folder.

## License
`hrtime` is licensed under the MIT license, for more information please
read the
[LICENSE file](https://github.com/marcelohdez/hrtime/blob/master/LICENSE).
