#![allow(dead_code)]

mod common;

use std::fs;

#[test]
fn test_isolation_roots_are_unique_and_strict() {
    let iso_a = common::TestIsolation::new("na0171_isolation");
    let iso_b = common::TestIsolation::new("na0171_isolation");

    assert_ne!(iso_a.root, iso_b.root, "test roots must be unique");
    assert!(iso_a.root.exists());
    assert!(iso_b.root.exists());

    let marker_a = iso_a.root.join("marker.txt");
    let marker_b = iso_b.root.join("marker.txt");
    fs::write(&marker_a, b"a").expect("write marker a");
    fs::write(&marker_b, b"b").expect("write marker b");

    assert_eq!(fs::read(&marker_a).expect("read marker a"), b"a");
    assert_eq!(fs::read(&marker_b).expect("read marker b"), b"b");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode_a = fs::metadata(&iso_a.root)
            .expect("metadata a")
            .permissions()
            .mode()
            & 0o777;
        let mode_b = fs::metadata(&iso_b.root)
            .expect("metadata b")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode_a, 0o700, "iso_a root perms must be 0700");
        assert_eq!(mode_b, 0o700, "iso_b root perms must be 0700");
    }
}
