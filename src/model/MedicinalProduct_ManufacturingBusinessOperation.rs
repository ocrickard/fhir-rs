#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).

#[derive(Debug)]
pub struct MedicinalProduct_ManufacturingBusinessOperation<'a> {
    pub value: &'a Value,
}

impl MedicinalProduct_ManufacturingBusinessOperation<'_> {
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

    /// Regulatory authorization date.
    pub fn effective_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveDate") {
            return Some(string);
        }
        return None;
    }

    /// To indicate if this proces is commercially confidential.
    pub fn confidentiality_indicator(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("confidentialityIndicator") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The manufacturer or establishment associated with the process.
    pub fn manufacturer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturer") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Regulatory authorization reference number.
    pub fn authorisation_reference_number(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("authorisationReferenceNumber") {
            return Some(Identifier { value: val });
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

    /// The type of manufacturing operation.
    pub fn operation_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("operationType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for effectiveDate
    pub fn _effective_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A regulator which oversees the operation.
    pub fn regulator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("regulator") {
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.effective_date() {}
        if let Some(_val) = self.confidentiality_indicator() {
            _val.validate();
        }
        if let Some(_val) = self.manufacturer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.authorisation_reference_number() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.operation_type() {
            _val.validate();
        }
        if let Some(_val) = self._effective_date() {
            _val.validate();
        }
        if let Some(_val) = self.regulator() {
            _val.validate();
        }
        return true;
    }
}
