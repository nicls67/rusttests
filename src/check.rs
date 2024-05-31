//!
//! Module containing check methods
//!

use core::fmt::Debug;

/// Check type (equality between values, different values)
pub enum CheckType {
    Equal,
    Different,
    Superior,
    SupOrEqual,
    Inferior,
    InfOrEqual
}

type Id = (u8,u8);

///
/// Checks if the `Result` value contains the expected variant.
/// If the `Ok` variant is expected and the check is Passed, the value contained in the `Ok` variant is returned. If an `Err` is expected and the check is Passed, `None` is returned
pub fn check_result<T,U>(id: Id, obtained: Result<T, U>, expected_ok: bool) -> Result<Option<T>, String> {
    match expected_ok {
        true => {
            match obtained {
                Ok(a) => Ok(Some(a)),
                Err(_) => Err(format!("Step {}.{} : Obtained result should be Ok, got Err", id.0, id.1)),
            }
        },
        false => {
            match obtained {
                Ok(_) => Err(format!("Step {}.{} : Obtained result should be Err, got Ok", id.0, id.1)),
                Err(_) => Ok(None),
            }
        },
    }
}

///
/// Checks if the `Option` value contains the expected variant.
/// If the `Some` variant is expected and the check is Passed, the value contained in the `Some` variant is returned. If an `None` is expected and the check is Passed, `None` is returned
pub fn check_option<T>(id: Id, obtained: Option<T>, expected_some: bool) -> Result<Option<T>, String> {
    match expected_some {
        true => {
            match obtained {
                Some(a) => Ok(Some(a)),
                None => Err(format!("Step {}.{} : Obtained result should be Some, got None", id.0, id.1)),
            }
        },
        false => {
            match obtained {
                Some(_) => Err(format!("Step {}.{} : Obtained result should be None, got Some", id.0, id.1)),
                None => Ok(None),
            }
        },
    }
}

///
/// Structure equality check
pub fn check_struct<T: PartialEq + Debug>(id: Id, obtained: &T, expected: &T, check_type: CheckType) -> Result<(), String> {
    match check_type {
        CheckType::Equal => {
            if obtained == expected {
                Ok(())
            } else {
                Err(format!("Step {}.{} : Values are not equal => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            }
        },
        CheckType::Different => {
            if obtained == expected {
                Err(format!("Step {}.{} : Values are not different => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            } else {
                Ok(())
            }
        },
        _ => Err(format!("Check type not consistant"))
    }
}

///
/// Value check
pub fn check_value<T: PartialEq + PartialOrd + Debug>(id: Id, obtained: &T, expected: &T, check_type: CheckType) -> Result<(), String> {
    match check_type {
        CheckType::Equal => {
            if obtained == expected {
                Ok(())
            } else {
                Err(format!("Step {}.{} : Values are not equal => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            }
        },
        CheckType::Different => {
            if obtained == expected {
                Err(format!("Step {}.{} : Values are not different => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            } else {
                Ok(())
            }
        },
        CheckType::Superior => {
            if obtained > expected {
                Ok(())
            } else {
                Err(format!("Step {}.{} : Obtained value is not superior to expected => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            }
        },
        CheckType::SupOrEqual => {
            if obtained >= expected {
                Ok(())
            } else {
                Err(format!("Step {}.{} : Obtained value is not superior or equal to expected => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            }
        },
        CheckType::Inferior => {
            if obtained < expected {
                Ok(())
            } else {
                Err(format!("Step {}.{} : Obtained value is not inferior to expected => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            }
        },
        CheckType::InfOrEqual => {
            if obtained < expected {
                Ok(())
            } else {
                Err(format!("Step {}.{} : Obtained value is not inferior or equal to expected => Expected value: {:?} => Obtained value: {:?}", id.0, id.1, expected, obtained))
            }
        },
    }
}