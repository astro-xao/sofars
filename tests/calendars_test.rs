#[cfg(test)]
mod tests {
    use sofars::cal::*;

    #[test]
    fn test_cal2jd() {
        let jd = cal2jd(2003, 6, 1).unwrap();
        assert_eq!(jd.0, 2400000.5);
        assert_eq!(jd.1, 52791.0);
    }
}
