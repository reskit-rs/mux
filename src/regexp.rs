use regex::Regex;
use anyhow::anyhow;
use percent_encoding::percent_decode;
use std::borrow::Cow;

struct RouteRegexpOptions {
	strict_slash:    bool,
	use_encoded_path: bool,
}

/// RegexpType defines the regexp type
#[allow(unused_qualifications)]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum RegexpType {
    Path,
    Host,
    Prefix,
    Query,
}

// RouteRegexp stores a regexp to match a host or path and information to
// collect and validate route variables.
struct RouteRegexp<'a> {
    tempalte: String,
    regexp_type: RegexpType,
    options: RouteRegexpOptions,
    regexp: &'a Regex,
    reverse: bool,
    vars_names: Vec<&'a str>,
    vars_regexps: Vec<&'a Regex>,
    wildcard_host_port: bool,
}

// RouteRegexpGroup groups the route matchers that carry variables.
struct RouteRegexpGroup<'a>  {
	host:    &'a RouteRegexp<'a>,
	path:    &'a RouteRegexp<'a>,
	queries: Vec<&'a RouteRegexp<'a>>,
}

// find_first_query_key returns the same result as (*url.URL).Query()[key][0].
// If key was not found, empty string and false is returned.
fn find_first_query_key<'a>(raw_query: &'a str, key: &str) -> (Cow<'a, str>, bool) {
    let mut query = raw_query;
	while !query.is_empty() {
		let mut found_key = query;
        match found_key.split_once(|c| c == '&' || c == ';') {
            Some(result) => {
                found_key = result.0;
                query = result.1;
            }
            None => {
                query = "";
            }
        }
		if found_key.len() == 0 {
			continue
		}
        let mut value = "";
        match found_key.split_once('=') {
            Some(result) => {
                found_key = result.0;
                value = result.1;
            }
            None => {}
        }
		if found_key.len() < key.len() {
			// Cannot possibly be key.
			continue
		}
		if let Ok(key_string) = percent_decode(found_key.as_bytes()).decode_utf8() {
            if key_string != key {
                continue
            }
        } else {
            continue
        }
        if let Ok(value_string) = percent_decode(value.as_bytes()).decode_utf8() {
            return (value_string, true)
        } else {
            continue
        }
	}
	(Cow::Borrowed(""), false)
}

// brace_indices returns the first level curly brace indices from a string.
// It returns an error in case of unbalanced braces.
fn brace_indices(s: &str) -> anyhow::Result<Vec<usize>> {
    let mut idxs: Vec<usize> = Vec::new();
    let mut level: isize =0;
    let mut idx: usize =0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '{' => {
                level += 1;
                if level == 1 {
                    idx = i;
                }
            },
            '}' => {
                level -= 1;
                if level == 0 {
                    idxs.push(idx);
                    idxs.push(i+1);
                }
                if level < 0 {
                    return Err(anyhow!("mux: unbalanced braces in {}", s));
                }
            },
            _ => {},
        }
    }
	Ok(idxs)
}

#[cfg(test)]
mod tests {
    use super::{RegexpType, find_first_query_key};

    #[test]
    fn test_regexp_type() {
        assert_eq!(RegexpType::Path as u8, 0);
        assert_eq!(RegexpType::Host as u8, 1);
        assert_eq!(RegexpType::Prefix as u8, 2);
        assert_eq!(RegexpType::Query as u8, 3);
    }

    #[test]
    fn test_find_first_query_key() {
        // use std::collections::HashMap;
        // use url::Url;
        // let parsed_url = Url::parse("http://example.com/?a=1&a=3&b=2").unwrap();
        // let params: HashMap<_, _> = parsed_url.query_pairs().collect();
        use std::borrow::Cow;
        assert_eq!(find_first_query_key("a=1&a=3&b=2", "a"), (Cow::Borrowed("1"), true));
        assert_eq!(find_first_query_key("a=1&a=3&b=2", "b"), (Cow::Borrowed("2"), true));
    }
}
