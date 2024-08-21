use rust_cfg_macro::feature;

#[test]
fn feature() {
    assert_eq!(feature!(if "foo" { 13 } else { 42 }), 42);
    assert_eq!(
        feature!(if "foo" {
            13
        } else if "bar" {
            13
        } else {
            42
        }),
        42
    );
    assert_eq!(
        feature!(if "foo" {
            13
        } else if "bar" {
            13
        } else if "baz" {
            13
        } else {
            42
        }),
        42
    );
}
