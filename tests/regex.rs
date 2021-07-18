use regex::{Regex,RegexSet};
use lazy_static::lazy_static;

lazy_static! {
    static ref DATE: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    static ref YMD: Regex = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    static ref DATESET: RegexSet = RegexSet::new(&[
        r"\d{4}-\d{2}-\d{2}",
        r"\d{2}/\d{2}/\d{4}",
    ]).unwrap();
}

fn match_date(text: &str) -> bool {
    DATE.is_match(text)
}

fn find_dates(text: &str) -> Vec<&str> {
    let mut dates: Vec<&str> = Vec::new();
    for cap in DATE.captures_iter(text) {
        dates.push(cap.get(0).map_or("", |m|m.as_str()));
    }
    dates
}

#[test]
fn regex_examples() {
    assert!(match_date("2014-01-01"));
    assert_eq!(find_dates("2012-03-14, 2013-01-01 and 2014-07-05"), vec!["2012-03-14", "2013-01-01", "2014-07-05"]);
    assert_eq!(YMD.replace_all("2012-03-14, 2013-01-01", "$m/$d/$y"), "03/14/2012, 01/01/2013");
    assert!(DATESET.is_match("2012-03-14"));
    assert!(DATESET.is_match("03/14/2012"));
}