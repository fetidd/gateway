use serde_json::Value;
use validify::{Validate, ValidationError, ValidationErrors};

pub type ExpectedValidationErrors = Vec<(
    ValidationErrorKind,        // ValidationError variant
    &'static str,               // Field name
    &'static str,               // Code
    &'static str,               // Message
    &'static str,               // Location
    Vec<(&'static str, Value)>, // Params (name, Value)
)>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValidationErrorKind { Field, Schema }

pub fn create_validation_errors(errors: ExpectedValidationErrors) -> ValidationErrors {
    errors
        .into_iter()
        .fold(ValidationErrors::new(), |mut errs, (t, n, c, m, l, p)| {
            let mut e = match t {
                ValidationErrorKind::Field => ValidationError::new_field_named(n, c),
                ValidationErrorKind::Schema => ValidationError::new_schema(c),
            }
            .with_message(m.into());
            if t == ValidationErrorKind::Field {
                for (k, v) in p.into_iter() {
                    e.add_param(k, &v);
                }
            }
            e.set_location(l);
            errs.add(e);
            errs
        })
}

pub fn check_validation<T: Validate>(t: T, errors: ExpectedValidationErrors) {
    if errors.len() == 0 {
        assert_eq!(t.validate(), Ok(()));
    } else {
        let exp = create_validation_errors(errors);
        assert_eq!(t.validate(), Err(exp));
    }
}
