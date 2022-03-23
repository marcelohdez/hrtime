fn main() {
    let time = "11:23:08";
    let sec = hrtime::to_sec(time);
    println!("{time} is {sec} seconds!");

    // All pass:
    assert_eq!(225, hrtime::to_sec("3:45"));
    assert_eq!(4320, hrtime::to_sec("1:12:00"));
    assert_eq!(18, hrtime::to_sec("18"));
    assert_eq!(19211, hrtime::to_sec("320:11"));
}