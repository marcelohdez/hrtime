mod tests;

/// Convert the given `secs` u64 into a readable time format without any
/// leading zeroes or colons, ex: `318` -> `"5:18"`.
/// 
/// If you would like leading zeroes on the output (ex: `1200` -> `"00:20:00"`)
/// you may use `from_sec`'s sister function, [`from_sec_padded`].
/// 
/// # Example:
/// ```
/// // Prints out "1200 seconds is 20:00"!
/// let mut sec = 1200;
/// println!("{sec} seconds is {},", hrtime::from_sec(sec));
/// // Passes:
/// assert_eq!("1:00", hrtime::from_sec(60));
/// ```
pub fn from_sec(secs: u64) -> String {
    let (hrs, min, sec) = to_time(secs);

    // 0>2 pads the number with 0s to the left if less than 2 digits wide
    if hrs > 0 { // If there are hours to show:
        format!("{hrs}:{min:0>2}:{sec:0>2}")
    } else if min > 0 { // Else if there are minutes to show:
        format!("{min}:{sec:0>2}")
    } else { // If there are only seconds to show:
        format!("{sec}")
    }
}

/// Much like [`from_sec`], this function will convert the given `secs` u64
/// into a readable time format. But unlike [`from_sec`], this function
/// will introduce leading 0's until it reaches the format `"00:00:00"`
/// 
/// # Example
/// ```
/// // Prints "28309 seconds with padding is 07:51:49"!
/// let sec = 28309;
/// println!("{sec} seconds with padding is {}", hrtime::from_sec_padded(sec));
/// // Passes:
/// assert_eq!("01:00:00", hrtime::from_sec_padded(3600));
/// ```
pub fn from_sec_padded(secs: u64) -> String {
    let (hrs, min, sec) = to_time(secs);
    // 0>2 pads the number with 0s to the left if less than 2 digits wide
    format!("{hrs:0>2}:{min:0>2}:{sec:0>2}")
}

/// Will attempt to convert the given `time` string into a u64 of seconds.
/// `time` must have the hours, minutes, and seconds separated by colons
/// respectively in order for this function to work correctly.
/// Ex: `"1:00"`, `"1:00:00"`, or `"2:38"`.
/// 
/// # Panics
/// Will panic if the given `time` string contains more than three colons
/// (ex: `"1:23:40:14"`), or if any of the time values before or after the
/// colons cannot be parsed as u64 integers (ex: `":23"` or `"12:"`).
/// 
/// # Examples
/// ## Hours, minutes, and seconds:
/// ```
/// // Prints "11:23:08 is 40988 seconds"!
/// let time = "11:23:08";
/// let sec = hrtime::to_sec(time);
/// println!("{time} is {sec} seconds");
/// // Passes:
/// assert_eq!(120, hrtime::to_sec("2:00"));
/// ```
/// 
/// ## Just seconds:
/// ```
/// // Prints "12 is 12 seconds"!
/// let time = "12";
/// let sec = hrtime::to_sec(time);
/// println!("{time} is {sec} seconds");
/// // Passes:
/// assert_eq!(48, hrtime::to_sec("48"));
/// ```
pub fn to_sec(time: &str) -> u64 {
    let mut sec = 0;
    let minpos = time.rfind(':'); // Position of minute colon (rightmost)

    // Check if minpos contains the index `i` of a colon:
    if let Some(i) = minpos {
        let hrpos = time[0..i].rfind(':'); // Position of hour colon (next colon from minute one)
        let mut start = 0;

        if let Some(pos) = hrpos { // If there is an hour colon:
            // Parse the value from the start to the hour colon
            sec += time[0..pos]
                    .parse::<u64>()
                    .unwrap()
                    * 60 * 60; // * 60 for min and * 60 for hr

            start = pos + 1;
        }

        // Add the parsed minute value
        sec += time[start..i]
                .parse::<u64>()
                .unwrap()
                * 60; // Convert into mins

        sec += time[i + 1..time.len()].parse::<u64>().unwrap(); // Convert text after this colon into seconds
    } else { // If there is no colon, treat text as only seconds:
        sec += time.parse::<u64>().unwrap();
    }

    sec
}

/// Returns the hours, minutes, and seconds respectively that are
/// represented in the given seconds `u64`.
/// 
/// ## Why are the minute and second values `u8`s?
/// Since both of these values will always return as <= 59, any larger
/// integer size is redundant. Therefore, if your program needs a
/// different integer range (outside of 0-255) remember to cast these
/// values as such!
/// 
/// # Example
/// ```
/// let seconds = 3661; // One hour, one minute, and one second.
/// let (hrs, min, sec) = hrtime::to_time(seconds);
/// // Prints "3661 seconds is 1h1m1s!"
/// println!("{seconds} seconds is {hrs}h{min}m{sec}s!");
/// 
/// // Passes:
/// assert_eq!((0, 2, 15), hrtime::to_time(135));
/// ```
pub fn to_time(secs: u64) -> (u64, u8, u8) {
    let sec = (secs % 60) as u8;
    let min = ((secs / 60) % 60) as u8;
    let hrs = secs / 60 / 60;

    (hrs, min, sec)
}
