use interop::safe::{Error, Kind, make_record, scale};

#[test]
fn by_value_struct_roundtrip() {
    let r = make_record(42, Kind::Tensor, 1.5);
    assert_eq!(r.id, 42);
    assert_eq!(r.kind, Kind::Tensor);
    assert_eq!(r.weight, 1.5);
}

#[test]
fn out_param_roundtrip() {
    let r = make_record(1, Kind::Scalar, 2.0);
    let scaled = scale(r, 3.0).unwrap();
    assert_eq!(scaled.weight, 6.0);
}

#[test]
fn c_side_rejects_bad_input() {
    let r = make_record(1, Kind::Scalar, 1.0);
    assert!(matches!(scale(r, -1.0), Err(Error::Range)));
}
