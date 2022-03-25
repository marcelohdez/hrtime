fn main() {
    let seconds = 301;
    let (hrs, min, sec) = hrtime::to_time(seconds);
    // Prints "301 seconds is 0h5m1s!"
    println!("{seconds} seconds is {hrs}h{min}m{sec}s!");
}