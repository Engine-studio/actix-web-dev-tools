pub(crate) mod db;
pub(crate) mod models;
pub use self::db::{
    Auth,
    AuthSecret
};

use super::error::{
    Result,
    ApiError,
    ErrorType,
};

#[macro_export]
macro_rules! require {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr,$msg:literal) => {
        // `stringify!` will convert the expression *as it is* into a string.

        if !$expression {
            return Err(ApiError{
                code: 403,
                message: stringify!($literal).to_string(),
                error_type: ErrorType::InternalError,
            });
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::error::ApiError;

    fn get_res_err() -> Result<()> {
        require!(false,"str");
        Ok(())
    }
    fn get_res_ok() -> Result<()> {
        require!(true,"str");
        Ok(())
    }
    use super::*;
    #[test]
    fn assert_is_error() {
        assert!(get_res_err().is_err());
    }
    #[test]
    fn assert_is_ok() {
        assert!(get_res_ok().is_ok());
    }
}
