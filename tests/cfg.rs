use cfg_exif::cfg;

#[test]
fn cfg() {
    assert_eq!(cfg!(if feature == "foo" { 13 } else { 42 }), 42);
}
