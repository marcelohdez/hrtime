fn main() {
    let mut sec = 1200;
    println!("{sec} seconds is {},", hrtime::from_sec(sec));
    println!("{sec} with padding is {},", hrtime::from_sec_padded(sec));

    sec = 28309;
    println!("{sec} is {},", hrtime::from_sec(sec));
    println!("and {sec} with padding is {}!", hrtime::from_sec_padded(sec));

    // All pass:
    assert_eq!("2:57", hrtime::from_sec(177));
    assert_eq!("1:01:02", hrtime::from_sec(3662));
    assert_eq!("03:02:57", hrtime::from_sec_padded(10977));
    assert_eq!("00:00:00", hrtime::from_sec_padded(0));
}