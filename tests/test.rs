extern crate leap_year;

#[test]
fn test_leap_year() {
    assert!(!leap_year::is_leap_year(2100), "2100 is not a leap year");
    assert!(!leap_year::is_leap_year(2101), "2101 is not a leap year");
    assert!(leap_year::is_leap_year(2000), "2000 is a leap year");
    assert!(leap_year::is_leap_year(2016), "2016 is a leap year");
}
