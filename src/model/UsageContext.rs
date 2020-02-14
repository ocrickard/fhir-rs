#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Specifies clinical/business/etc. metadata that can be used to retrieve, index
/// and/or categorize an artifact. This metadata can either be specific to the
/// applicable population (e.g., age category, DRG) or the specific context of care
/// (e.g., venue, care setting, provider of care).

#[derive(Debug)]
pub struct UsageContext<'a> {
    pub value: &'a Value,
}

impl UsageContext<'_> {
    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A code that identifies the type of context being specified by this usage
    /// context.
    pub fn code(&self) -> Coding {
        Coding {
            value: &self.value["code"],
        }
    }

    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept { value: val });
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
        if let Some(_val) = self.value_reference() {
            _val.validate();
        }
        let _ = self.code().validate();
        if let Some(_val) = self.value_range() {
            _val.validate();
        }
        if let Some(_val) = self.value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.value_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
