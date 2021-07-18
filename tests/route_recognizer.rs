use route_recognizer::{Params, Router};

#[test]
fn wildcard_colon() {
    let mut router = Router::new();

    router.add("/a/*b", "ab".to_string());
    router.add("/a/*b/c", "abc".to_string());
    router.add("/a/*b/c/*d", "abcd".to_string()); // NOTE: 支持两个通配符！！！

    let m = router.recognize("/a/foo").unwrap();
    assert_eq!(*m.handler(), &"ab".to_string());
    assert_eq!(m.params(), &params("b", "foo"));

    let m = router.recognize("/a/foo/bar").unwrap();
    assert_eq!(*m.handler(), &"ab".to_string());
    assert_eq!(m.params(), &params("b", "foo/bar"));

    let m = router.recognize("/a/foo/c").unwrap();
    assert_eq!(*m.handler(), &"abc".to_string());
    assert_eq!(m.params(), &params("b", "foo"));

    let m = router.recognize("/a/foo/bar/c").unwrap();
    assert_eq!(*m.handler(), &"abc".to_string());
    assert_eq!(m.params(), &params("b", "foo/bar"));

    let m = router.recognize("/a/foo/c/baz").unwrap();
    assert_eq!(*m.handler(), &"abcd".to_string());
    assert_eq!(m.params(), &two_params("b", "foo", "d", "baz"));

    let m = router.recognize("/a/foo/bar/c/baz/bay").unwrap();
    assert_eq!(*m.handler(), &"abcd".to_string());
    assert_eq!(m.params(), &two_params("b", "foo/bar", "d", "baz/bay"));
}


fn params(key: &str, val: &str) -> Params {
    let mut map = Params::new();
    map.insert(key.to_string(), val.to_string());
    map
}

fn two_params(k1: &str, v1: &str, k2: &str, v2: &str) -> Params {
    let mut map = Params::new();
    map.insert(k1.to_string(), v1.to_string());
    map.insert(k2.to_string(), v2.to_string());
    map
}