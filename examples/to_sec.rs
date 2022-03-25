fn main() {
    let time = "11:23:08";
    let sec = hrtime::to_sec(time);
    println!("{time} is {sec} seconds!");
}