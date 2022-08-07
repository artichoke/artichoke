use scolapasta_int_parse::InvalidRadixExceptionKind;

use crate::extn::prelude::*;

impl<'a> From<scolapasta_int_parse::Error<'a>> for Error {
    fn from(err: scolapasta_int_parse::Error<'a>) -> Self {
        use scolapasta_int_parse::Error::{Argument, Radix};

        match err {
            Argument(err) => {
                let message = err.to_string();
                ArgumentError::from(message).into()
            }
            Radix(err) => match err.exception_kind() {
                InvalidRadixExceptionKind::ArgumentError => {
                    let message = err.to_string();
                    ArgumentError::from(message).into()
                }
                InvalidRadixExceptionKind::RangeError => {
                    let message = err.to_string();
                    RangeError::from(message).into()
                }
            },
        }
    }
}
