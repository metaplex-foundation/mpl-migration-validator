#[macro_export]
macro_rules! assert_custom_error_ix {
    ($ix:expr, $error:expr, $matcher:pat) => {
        match $error {
            BanksClientError::TransactionError(TransactionError::InstructionError(
                $ix,
                InstructionError::Custom(x),
            )) => match FromPrimitive::from_i32(x as i32) {
                Some($matcher) => assert!(true),
                Some(other) => {
                    assert!(
                        false,
                        "Expected another custom instruction error than '{:#?}'",
                        other
                    )
                }
                None => assert!(false, "Expected custom instruction error"),
            },
            err => assert!(
                false,
                "Expected custom instruction error but got '{:#?}'",
                err
            ),
        };
    };
}
