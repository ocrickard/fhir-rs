#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// The shelf-life and storage information for a medicinal product item or container
/// can be described using this class.

#[derive(Debug)]
pub struct ProductShelfLife<'a> {
    pub value: &'a Value,
}

impl ProductShelfLife<'_> {
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

    /// Unique identifier for the packaged Medicinal Product.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Special precautions for storage, if any, can be specified using an appropriate
    /// controlled vocabulary The controlled term and the controlled term identifier
    /// shall be specified.
    pub fn special_precautions_for_storage(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialPrecautionsForStorage") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The shelf life time period can be specified using a numerical value for the
    /// period of time and its unit of time measurement The unit of measurement shall be
    /// specified in accordance with ISO 11240 and the resulting terminology The symbol
    /// and the symbol identifier shall be used.
    pub fn period(&self) -> Quantity {
        Quantity {
            value: &self.value["period"],
        }
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// This describes the shelf life, taking into account various scenarios such as
    /// shelf life of the packaged Medicinal Product itself, shelf life after
    /// transformation where necessary and shelf life after the first opening of a
    /// bottle, etc. The shelf life type shall be specified using an appropriate
    /// controlled vocabulary The controlled term and the controlled term identifier
    /// shall be specified.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.special_precautions_for_storage() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.period().validate();
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        let _ = self.fhir_type().validate();
        return true;
    }
}
