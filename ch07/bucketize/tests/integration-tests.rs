extern crate bucketize;
use bucketize::Bucketizer;

#[test]
fn complex_bucketizer() {
    let b = Bucketizer::new()
        .bucket(None, Some(-100.0), -100.0)
        .bucket(Some(-100.0), Some(0.0), -50.0)
        .bucket(Some(0.0), Some(100.0), 50.0)
        .bucket(Some(100.0), None, 100.0);

    assert_eq!(b.bucketize(37.4), Some(50.0));
}
