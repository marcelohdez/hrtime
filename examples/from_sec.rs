fn main() {
    let mut sec = 1200;
    println!("{sec} seconds is {},", hrtime::from_sec(sec));
    println!("{sec} with padding is {},", hrtime::from_sec_padded(sec));

    sec = 28309;
    println!("{sec} is {},", hrtime::from_sec(sec));
    println!("and {sec} with padding is {}!", hrtime::from_sec_padded(sec));
}