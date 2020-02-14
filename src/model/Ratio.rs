#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// A relationship of two Quantity values - expressed as a numerator and a
/// denominator.

#[derive(Debug)]
pub struct Ratio<'a> {
    pub value: &'a Value,
}

impl Ratio<'_> {
    /// The value of the numerator.
    pub fn numerator(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("numerator") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The value of the denominator.
    pub fn denominator(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("denominator") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.numerator() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.denominator() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
