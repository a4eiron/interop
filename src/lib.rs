mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::{SHIM_ABI_VERSION, shim_record, shim_status};

pub mod safe {
    use super::ffi;
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Record {
        pub id: u64,
        pub kind: Kind,
        pub weight: f64,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Kind {
        Scalar,
        Tensor,
    }

    impl Kind {
        fn to_c(self) -> u32 {
            match self {
                Self::Scalar => 1,
                Self::Tensor => 2,
            }
        }
    }

    #[derive(Debug)]
    pub enum Error {
        Null,
        Range,
        Internal,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Null => write!(f, "null pointer"),
                Self::Range => write!(f, "argument out of range"),
                Self::Internal => write!(f, "internal C error"),
            }
        }
    }
    impl std::error::Error for Error {}

    fn status_to_result(s: ffi::shim_status) -> Result<(), Error> {
        match s {
            ffi::shim_status_SHIM_OK => Ok(()),
            ffi::shim_status_SHIM_ERR_NULL => Err(Error::Null),
            ffi::shim_status_SHIM_ERR_RANGE => Err(Error::Range),
            _ => Err(Error::Internal),
        }
    }

    pub fn make_record(id: u64, kind: Kind, weight: f64) -> Record {
        let raw = unsafe { ffi::shim_make_record(id, kind.to_c(), weight) };
        Record {
            id: raw.id,
            kind,
            weight: raw.weight,
        }
    }

    pub fn scale(r: Record, factor: f64) -> Result<Record, Error> {
        let mut raw = ffi::shim_record {
            id: r.id,
            kind: r.kind.to_c(),
            flags: 0,
            weight: r.weight,
        };

        let st = unsafe { ffi::shim_scale_record(&mut raw, factor) };
        status_to_result(st)?;
        Ok(Record {
            id: raw.id,
            kind: r.kind,
            weight: raw.weight,
        })
    }
}

pub fn c_abi_version() -> u32 {
    unsafe { ffi::shim_abi_version() }
}
