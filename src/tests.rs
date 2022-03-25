#[cfg(test)]
mod tests {
    // ======= from_sec =======
    #[test]
    fn test_from_sec_sec() {
        let result = crate::from_sec(43);
        assert_eq!(&result, "43");
    }

    #[test]
    fn test_from_sec_min() {
        let result = crate::from_sec(100);
        assert_eq!(&result, "1:40");
    }

    #[test]
    fn test_from_sec_hr() {
        let result = crate::from_sec(3671);
        assert_eq!(&result, "1:01:11");
    }

    // ======= from_sec_padded =======
    #[test]
    fn test_from_sec_padded_sec() {
        let result = crate::from_sec_padded(55);
        assert_eq!(&result, "00:00:55");
    }

    #[test]
    fn test_from_sec_padded_min() {
        let result = crate::from_sec_padded(892);
        assert_eq!(&result, "00:14:52");
    }

    #[test]
    fn test_from_sec_padded_hrs() {
        let result = crate::from_sec_padded(3999);
        assert_eq!(&result, "01:06:39");
    }

    // ======= to_sec =======
    #[test]
    fn test_to_sec_sec() {
        let time = "2";
        let result = crate::to_sec(time);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_to_sec_min() {
        let time = "37:18";
        let result = crate::to_sec(time);
        assert_eq!(result, 2238);
    }

    #[test]
    fn test_to_sec_hrs() {
        let time = "8:29:17";
        let result = crate::to_sec(time);
        assert_eq!(result, 30557);
    }

    // ======= to_time =======
    #[test]
    fn test_to_time_sec() {
        let seconds = 40;
        let result = crate::to_time(seconds);
        assert_eq!(result, (0, 0, 40));
    }

    #[test]
    fn test_to_time_min() {
        let seconds = 378; // 6 minutes, 18 seconds
        let result = crate::to_time(seconds);
        assert_eq!(result, (0, 6, 18));
    }

    #[test]
    fn test_to_time_hr() {
        let seconds = 3710; // 1 hour, 1 minute, 50 seconds
        let result = crate::to_time(seconds);
        assert_eq!(result, (1, 1, 50));
    }
}
