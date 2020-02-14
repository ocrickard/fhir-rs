#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use serde_json::value::Value;

/// Indicates how the medication is/was taken or should be taken by the patient.

#[derive(Debug)]
pub struct Dosage_DoseAndRate<'a> {
    pub value: &'a Value,
}

impl Dosage_DoseAndRate<'_> {
    /// Amount of medication per unit of time.
    pub fn rate_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("rateQuantity") {
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

    /// Amount of medication per unit of time.
    pub fn rate_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("rateRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Amount of medication per dose.
    pub fn dose_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("doseRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// The kind of dose or rate specified, for example, ordered or calculated.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Amount of medication per dose.
    pub fn dose_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("doseQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Amount of medication per unit of time.
    pub fn rate_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("rateRatio") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.rate_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.rate_range() {
            _val.validate();
        }
        if let Some(_val) = self.dose_range() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.dose_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.rate_ratio() {
            _val.validate();
        }
        return true;
    }
}
