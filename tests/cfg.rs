use cfg_exif::cfg;

#[test]
fn feature() {
    assert_eq!(cfg!(if (feature == "foo") { 13 } else { 42 }), 42);
    assert_eq!(
        cfg!(if (feature == "foo") {
            13
        } else if (feature == "bar") {
            13
        } else {
            42
        }),
        42
    );
}

#[test]
fn target_os() {
    assert_eq!(cfg!(if (target_os == "windows") { 13 } else { 42 }), 42);
    assert_eq!(
        cfg!(if (target_os == "windows") {
            13
        } else if (target_os == "fuchsia") {
            13
        } else {
            42
        }),
        42
    );
}

#[test]
fn not() {
    assert_eq!(cfg!(if (not(feature == "foo")) { 42 } else { 13 }), 42);
    assert_eq!(
        cfg!(if (not(feature == "foo")) {
            42
        } else if (not(feature == "bar")) {
            42
        } else {
            13
        }),
        42
    );
    assert_eq!(
        cfg!(if (target_os == "fuchsia") {
            0
        } else if (feature == "foo") {
            1
        } else {
            42
        }),
        42
    );
}
