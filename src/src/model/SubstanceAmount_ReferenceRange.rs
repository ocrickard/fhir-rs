#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Chemical substances are a single substance type whose primary defining element
/// is the molecular structure. Chemical substances shall be defined on the basis of
/// their complete covalent molecular structure; the presence of a salt (counter-
/// ion) and/or solvates (water, alcohols) is also captured. Purity, grade,
/// physical form or particle size are not taken into account in the definition of a
/// chemical substance or in the assignment of a Substance ID.

#[derive(Debug)]
pub struct SubstanceAmount_ReferenceRange<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceAmount_ReferenceRange<'_> {
    pub fn new(value: &Value) -> SubstanceAmount_ReferenceRange {
        SubstanceAmount_ReferenceRange {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Upper limit possible or expected.
    pub fn high_limit(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("highLimit") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
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

    /// Lower limit possible or expected.
    pub fn low_limit(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("lowLimit") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.high_limit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.low_limit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceAmount_ReferenceRangeBuilder {
    pub(crate) value: Value,
}

impl SubstanceAmount_ReferenceRangeBuilder {
    pub fn build(&self) -> SubstanceAmount_ReferenceRange {
        SubstanceAmount_ReferenceRange {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceAmount_ReferenceRange) -> SubstanceAmount_ReferenceRangeBuilder {
        SubstanceAmount_ReferenceRangeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceAmount_ReferenceRangeBuilder {
        let mut __value: Value = json!({});
        return SubstanceAmount_ReferenceRangeBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceAmount_ReferenceRangeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn high_limit<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SubstanceAmount_ReferenceRangeBuilder {
        self.value["highLimit"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceAmount_ReferenceRangeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn low_limit<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SubstanceAmount_ReferenceRangeBuilder {
        self.value["lowLimit"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceAmount_ReferenceRangeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
