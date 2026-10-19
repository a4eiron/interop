use interop::{SHIM_ABI_VERSION, shim_record};

#[test]
fn struct_size_and_align_match_expected_x86_64_sysv() {
    assert_eq!(std::mem::size_of::<shim_record>(), 24);
    assert_eq!(std::mem::align_of::<shim_record>(), 8);
}

#[test]
fn c_and_rust_agree_on_abi_version() {
    assert_eq!(interop::c_abi_version(), SHIM_ABI_VERSION)
}

#[test]
fn field_offsets_are_stable() {
    let r = shim_record {
        id: 0,
        kind: 0,
        flags: 0,
        weight: 0.0,
    };
    let base = &r as *const _ as usize;
    assert_eq!(&r.id as *const _ as usize - base, 0);
    assert_eq!(&r.kind as *const _ as usize - base, 8);
    assert_eq!(&r.flags as *const _ as usize - base, 12);
    assert_eq!(&r.weight as *const _ as usize - base, 16);
}
